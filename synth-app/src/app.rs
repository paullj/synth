use crate::{
    input::{get_first_midi_device, start_midi_input_thread},
    output::start_output_thread,
    state::{
        compose::ComposeScreen,
        edit::EditScreen,
        error::ErrorScreen,
        mode::{Mode, ModeScreen},
        play::PlayScreen,
        startup::StartupScreen,
        Event, Machine, Screen,
    },
};
use crossbeam::{atomic::AtomicCell, queue::SegQueue};
use fundsp::shared::Shared;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};

use midir::MidiInput;
use std::sync::Arc;
use wmidi::Note;

pub struct App {
    machine: Machine,
    state: Arc<State>,
    action_messages: Arc<SegQueue<ActionMessage>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StateParameter {
    Control1,
    Control2,
    Control3,
    Control4,
    Attack,
    Decay,
    Sustain,
    Release,
    FilterCutoff,
    FilterType,
    Reverb,
    Delay,
    Chorus,
    VibratoDepth,
    VibratoRate,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Direction {
    Increment = 1,
    Decrement = -1,
}

impl std::ops::Add<Direction> for f64 {
    type Output = f64;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::Increment => 1.0,
            Direction::Decrement => -1.0,
        }
    }
}

impl std::ops::Mul<Direction> for f64 {
    type Output = f64;

    fn mul(self, rhs: Direction) -> Self::Output {
        self * match rhs {
            Direction::Increment => 1.0,
            Direction::Decrement => -1.0,
        }
    }
}

pub(crate) struct State {
    pub(crate) control_1: Shared<f64>,
    pub(crate) control_2: Shared<f64>,
    pub(crate) control_3: Shared<f64>,
    pub(crate) control_4: Shared<f64>,

    pub(crate) attack: Shared<f64>,
    pub(crate) decay: Shared<f64>,
    pub(crate) sustain: Shared<f64>,
    pub(crate) release: Shared<f64>,

    pub(crate) filter_cutoff: Shared<f64>,
    pub(crate) filter_type: Shared<f64>,
    // lfo?
    // envelope?
    pub(crate) vibrato_rate: Shared<f64>,
    pub(crate) vibrato_depth: Shared<f64>,
    pub(crate) reverb: Shared<f64>,
    pub(crate) delay: Shared<f64>,
    pub(crate) chorus: Shared<f64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            attack: Shared::new(0.2),
            decay: Shared::new(0.2),
            sustain: Shared::new(0.9),
            release: Shared::new(0.7),
            control_1: Shared::new(0.5),
            control_2: Shared::new(0.5),
            control_3: Shared::new(0.5),
            control_4: Shared::new(0.5),
            filter_cutoff: Shared::new(0.1),
            filter_type: Shared::new(0.1),
            reverb: Shared::new(0.0),
            delay: Shared::new(0.0),
            chorus: Shared::new(0.0),
            vibrato_depth: Shared::new(0.0),
            vibrato_rate: Shared::new(0.0),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum EngineMessage {
    NoteOn(Note),
    NoteOff(Note),
    ChangeParameter(Direction, StateParameter),
}

pub(crate) enum ActionMessage {
    A,
    B,
    X,
    Y,
    Quit,
}

impl App {
    pub fn new() -> Self {
        let mut input = MidiInput::new("Synth MIDI Input").unwrap();
        let input_port = get_first_midi_device(&mut input).unwrap();

        let state = Arc::new(State::default());
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
        start_output_thread(state.clone(), engine_messages.clone());

        let mut initial_state = StartupScreen::default();
        initial_state.entry();

        Self {
            state,
            action_messages,
            machine: Machine::Startup(initial_state),
        }
    }

    pub(crate) fn next(&mut self, event: Event) {
        // TODO: neaten this up
        let next_state = match (&self.machine, event) {
            (Machine::Startup(_), Event::Initialized) => Some(Machine::Play(PlayScreen::default())),
            (_, Event::Quit) => None,
            (_, Event::Error(message)) => Some(Machine::Error(ErrorScreen { message })),
            (Machine::Mode(prev), Event::CloseModeMenu) => match prev.selected_mode {
                Mode::Play => Some(Machine::Play(PlayScreen::default())),
                Mode::Compose => Some(Machine::Compose(ComposeScreen {})),
                Mode::Edit => Some(Machine::Edit(EditScreen {})),
            },
            (Machine::Play(_), Event::OpenModeMenu) => Some(Machine::Mode(ModeScreen {
                selected_mode: Mode::Play,
            })),
            (Machine::Compose(_), Event::OpenModeMenu) => Some(Machine::Mode(ModeScreen {
                selected_mode: Mode::Compose,
            })),
            (Machine::Edit(_), Event::OpenModeMenu) => Some(Machine::Mode(ModeScreen {
                selected_mode: Mode::Edit,
            })),
            (_, _) => Some(Machine::Error(ErrorScreen {
                message: String::from("Unknown error from unhandled event"),
            })),
        };
        if let Some(next_state) = next_state {
            self.handle_exit();
            self.machine = next_state;
            self.handle_entry();
        }
    }

    fn handle_entry(&mut self) {
        match &mut self.machine {
            Machine::Startup(screen) => screen.entry(),
            Machine::Play(screen) => screen.entry(),
            Machine::Mode(screen) => screen.entry(),
            Machine::Compose(screen) => screen.entry(),
            Machine::Edit(screen) => screen.entry(),
            Machine::Error(screen) => screen.entry(),
        }
    }

    fn handle_exit(&mut self) {
        match &mut self.machine {
            Machine::Startup(screen) => screen.exit(),
            Machine::Play(screen) => screen.exit(),
            Machine::Mode(screen) => screen.exit(),
            Machine::Compose(screen) => screen.exit(),
            Machine::Edit(screen) => screen.exit(),
            Machine::Error(screen) => screen.exit(),
        }
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), anyhow::Error>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        let ref state = self.state;
        match &self.machine {
            Machine::Startup(screen) => screen.draw(target, state)?,
            Machine::Mode(screen) => screen.draw(target, state)?,
            Machine::Play(screen) => screen.draw(target, state)?,
            Machine::Compose(screen) => screen.draw(target, state)?,
            Machine::Edit(screen) => screen.draw(target, state)?,
            Machine::Error(screen) => screen.draw(target, state)?,
        }

        // TODO: Remove this, just for debugging
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::MAGENTA);
        let text = Text::new(&self.machine.to_string(), Point::new(1, 6), style).draw(target);
        match text {
            Ok(_) => {}
            Err(_) => panic!("Error drawing text"),
        };
        Ok(())
    }

    pub fn update(&mut self) -> () {
        let ref mut machine = self.machine;
        let actions = self.action_messages.clone();
        let ref state = self.state;
        let event = match machine {
            Machine::Startup(screen) => screen.update(state, actions),
            Machine::Mode(screen) => screen.update(state, actions),
            Machine::Play(screen) => screen.update(state, actions),
            Machine::Compose(screen) => screen.update(state, actions),
            Machine::Edit(screen) => screen.update(state, actions),
            Machine::Error(screen) => screen.update(state, actions),
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
