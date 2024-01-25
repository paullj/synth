use crate::{
    input::{get_first_midi_device, start_midi_input_thread},
    output::start_output_thread,
    state::{
        compose::ComposeScreen,
        edit::EditScreen,
        error::ErrorScreen,
        mode::{Mode, ModeScreen},
        play::PlayScreen,
        Event, Screen, State,
    },
};

use crossbeam::{atomic::AtomicCell, queue::SegQueue};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};

use midir::MidiInput;
use std::{fmt, sync::Arc};
use wmidi::Note;

#[derive(Debug)]
pub struct App {
    state: State,
    action_messages: Arc<SegQueue<ActionMessage>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EngineMessage {
    NoteOn(Note),
    NoteOff(Note),
    ControlChange(u8, u8, u8),
}

impl fmt::Display for EngineMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineMessage::NoteOn(note) => {
                write!(f, "NoteOn: note={}", note)
            }
            EngineMessage::NoteOff(note) => {
                write!(f, "NoteOff: note={}", note)
            }
            EngineMessage::ControlChange(channel, control, value) => write!(
                f,
                "ControlChange: channel={}, control={}, value={}",
                channel, control, value
            ),
        }
    }
}

pub(crate) enum ActionMessage {
    A,
    B,
    X,
    Y,
    Quit,
}

trait Draw<T: RgbColor> {
    fn draw<D>(&self, target: &mut D) -> ()
    where
        D: DrawTarget<Color = T>;
}

impl<T: RgbColor> Draw<T> for StartupScreen {
    fn draw<D>(&self, target: &mut D) -> ()
    where
        D: DrawTarget<Color = T>,
    {
        target.clear(T::MAGENTA);
    }
}

impl App {
    pub fn new() -> Self {
        let mut input = MidiInput::new("Synth MIDI Input").unwrap();
        let input_port = get_first_midi_device(&mut input).unwrap();

        let engine_messages = Arc::new(SegQueue::new());
        let action_messages = Arc::new(SegQueue::new());
        let quit = Arc::new(AtomicCell::new(false));

        start_midi_input_thread(
            input,
            input_port,
            engine_messages.clone(),
            action_messages.clone(),
            quit.clone(),
        );
        start_output_thread(engine_messages.clone());
        Self {
            action_messages,
            state: State::Mode(ModeScreen {
                selected_mode: Mode::Play,
            }),
        }
    }

    pub(crate) fn next(&mut self, event: Event) {
        // TODO: neaten this up
        let next_state = match (&self.state, event) {
            (State::Startup(_), Event::Initialized) => Some(State::Play(PlayScreen {})),
            (_, Event::Quit) => None,
            (_, Event::Error(message)) => Some(State::Error(ErrorScreen { message })),
            (State::Mode(prev), Event::CloseModeMenu) => match prev.selected_mode {
                Mode::Play => Some(State::Play(PlayScreen {})),
                Mode::Compose => Some(State::Compose(ComposeScreen {})),
                Mode::Edit => Some(State::Edit(EditScreen {})),
            },
            (State::Play(_), Event::OpenModeMenu) => Some(State::Mode(ModeScreen {
                selected_mode: Mode::Play,
            })),
            (State::Compose(_), Event::OpenModeMenu) => Some(State::Mode(ModeScreen {
                selected_mode: Mode::Compose,
            })),
            (State::Edit(_), Event::OpenModeMenu) => Some(State::Mode(ModeScreen {
                selected_mode: Mode::Edit,
            })),
            (_, _) => Some(State::Error(ErrorScreen {
                message: String::from("Unknown error from unhandled event"),
            })),
        };
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }

    pub fn draw<D>(&self, target: &mut D, time: f64, delta: f64) -> Result<(), anyhow::Error>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        match &self.state {
            State::Startup(screen) => screen.draw(target, time, delta)?,
            State::Mode(screen) => screen.draw(target, time, delta)?,
            State::Play(screen) => screen.draw(target, time, delta)?,
            State::Compose(screen) => screen.draw(target, time, delta)?,
            State::Edit(screen) => screen.draw(target, time, delta)?,
            State::Error(screen) => screen.draw(target, time, delta)?,
        }

        // TODO: Remove this, just for debugging
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::MAGENTA);
        let text = Text::new(&self.state.to_string(), Point::new(1, 6), style).draw(target);
        match text {
            Ok(_) => {}
            Err(_) => panic!("Error drawing text"),
        };
        Ok(())
    }

    pub fn update(&mut self, time: f64, delta: f64) -> () {
        let ref mut state = self.state;
        let event = match state {
            State::Startup(screen) => screen.update(self.action_messages.clone(), time, delta),
            State::Mode(screen) => screen.update(self.action_messages.clone(), time, delta),
            State::Play(screen) => screen.update(self.action_messages.clone(), time, delta),
            State::Compose(screen) => screen.update(self.action_messages.clone(), time, delta),
            State::Edit(screen) => screen.update(self.action_messages.clone(), time, delta),
            State::Error(screen) => screen.update(self.action_messages.clone(), time, delta),
        };

        if let Some(event) = event {
            self.next(event);

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup() {
        // let app = App::new();
        // assert_eq!(app.state, State::Startup(StartupScreen {}));
    }
}
