use crate::{
    input::{get_first_midi_device, start_midi_input_thread},
    output::start_output_thread,
};
use crossbeam::{atomic::AtomicCell, queue::SegQueue};
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
    pub fn run(&mut self) {
        match self.state {
            State::Startup(_) => {
                let mut input = MidiInput::new("MIDI Input").unwrap();
                let input_port = get_first_midi_device(&mut input).unwrap();

                let midi_messages = Arc::new(SegQueue::new());
                let quit = Arc::new(AtomicCell::new(false));

                let input_handle =
                    start_midi_input_thread(input, input_port, midi_messages.clone(), quit.clone());
                let output_handle = start_output_thread(midi_messages.clone());

                self.next(Event::Initialized);
            }
            State::Running => {
                println!("Playing");
            }
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
        let app = App::new();
        assert_eq!(app.state, State::Startup(StartupScreen {}));
    }
}
