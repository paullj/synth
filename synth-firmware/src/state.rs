use crate::key_code;
use defmt::Format;

#[derive(Debug, Clone, Format)]
pub struct State {
    pub octave: u8,
    pub notes_on: [bool; key_code::NUM_KEYS],
    pub positions: [u16; key_code::NUM_KEYS],
}

impl Default for State {
    fn default() -> Self {
        Self {
            octave: 4,
            notes_on: [false; key_code::NUM_KEYS],
            positions: [2000; key_code::NUM_KEYS],
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn octave_up(&mut self) {
        if self.octave < 8 {
            self.octave += 1;
        }
    }

    pub fn octave_down(&mut self) {
        if self.octave > 1 {
            self.octave -= 1;
        }
    }
}
