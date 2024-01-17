use crate::{
    input::{get_first_midi_device, start_input_thread},
    output::start_output_thread,
};

use std::sync::Arc;

use crossbeam::{atomic::AtomicCell, queue::SegQueue};
use midir::MidiInput;

#[derive(Debug, PartialEq)]
enum State {
    Startup,
    Play,
}
#[derive(Debug)]
enum Transition {
    Initialized,
}

#[derive(Debug)]
pub struct App {
    state: State,
}

trait Runnable {
    fn run(&mut self);
}

impl Runnable for State {
    fn run(&mut self) {
        match self {
            State::Startup => {
                println!("Starting up");
            }
            State::Play => {
                println!("Playing");
            }
        }
    }
}

impl App {
    /// Create a new App.
    pub fn new() -> Self {
        Self {
            state: State::Startup,
        }
    }
    fn transition(&mut self, transition: Transition) {
        match (&self.state, transition) {
            (State::Startup, Transition::Initialized) => {
                self.state = State::Play;
            }
            _ => panic!("Invalid transition"),
        }
    }
    pub fn run(&mut self) {
        match self.state {
            State::Startup => {
                let mut midi_in = MidiInput::new("MIDI Input").unwrap();
                let in_port = get_first_midi_device(&mut midi_in).unwrap();
                let midi_msgs = Arc::new(SegQueue::new());
                let quit = Arc::new(AtomicCell::new(false));
                start_input_thread(midi_msgs.clone(), midi_in, in_port, quit.clone());
                start_output_thread::<6>(midi_msgs.clone());
                self.transition(Transition::Initialized);
            }
            State::Play => {
                println!("Playing");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup() {
        let mut app = App::new();
        app.run();
        assert_eq!(app.state, State::Play);
    }
}
