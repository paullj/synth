use crate::{
    input::{get_first_midi_device, start_midi_input_thread},
    output::start_output_thread,
};
use crossbeam::{atomic::AtomicCell, queue::SegQueue};
use embedded_graphics::prelude::*;
use midir::MidiInput;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
enum State {
    Startup(StartupScreen),
    Running,
    Error,
}

#[derive(Debug)]
pub enum Event {
    Initialized,
    Quit,
}

#[derive(Debug)]
pub struct App {
    state: State,
}

#[derive(Debug, PartialEq)]
struct StartupScreen {}

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
        Self {
            state: State::Startup(StartupScreen {}),
        }
    }
    pub fn next(&mut self, event: Event) {
        match (&self.state, event) {
            (State::Startup(_), Event::Initialized) => {
                self.state = State::Running;
            }
            (_, Event::Quit) => {
                println!("Quitting");
            }
            (_, _) => self.state = State::Error,
        }
    }
    pub fn draw<D>(&self, target: &mut D) -> ()
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        match &self.state {
            State::Startup(screen) => screen.draw(target),
            _ => {}
        }
    }

    pub fn run(&mut self) {
        match &self.state {
            State::Startup(screen) => {
                let mut input = MidiInput::new("MIDI Input").unwrap();
                let input_port = get_first_midi_device(&mut input).unwrap();

                let messages = Arc::new(SegQueue::new());
                let quit = Arc::new(AtomicCell::new(false));

                let input_handle =
                    start_midi_input_thread(input, input_port, messages.clone(), quit.clone());
                let output_handle = start_output_thread(messages.clone());
                self.next(Event::Initialized);
            }
            State::Running => {}
            State::Error => {
                println!("Error");
            }
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
