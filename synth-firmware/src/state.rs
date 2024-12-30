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
}
