use wmidi::Note;

pub const NUM_KEYS: usize = 15;

#[derive(Clone, Copy)]
pub enum KeyCode {
    SHIFT = 0,
    UP = 1,
    DOWN = 2,
    C1 = 3,
    CSharp1 = 4,
    D1 = 5,
    DSharp1 = 6,
    E1 = 7,
    F1 = 8,
    FSharp1 = 9,
    G1 = 10,
    GSharp1 = 11,
    A1 = 12,
    ASharp1 = 13,
    B1 = 14,
}

impl KeyCode {
    pub fn to_note(&self, octave: u8) -> Option<Note> {
        if octave < 1 || octave > 8 {
            return None;
        }

        if let Some(note) = match &self {
            Self::C1 => Some(Note::C1),
            Self::CSharp1 => Some(Note::CSharp1),
            Self::D1 => Some(Note::D1),
            Self::DSharp1 => Some(Note::DSharp1),
            Self::E1 => Some(Note::E1),
            Self::F1 => Some(Note::F1),
            Self::FSharp1 => Some(Note::FSharp1),
            Self::G1 => Some(Note::G1),
            Self::GSharp1 => Some(Note::GSharp1),
            Self::A1 => Some(Note::A1),
            Self::ASharp1 => Some(Note::ASharp1),
            Self::B1 => Some(Note::B1),
            _ => None,
        } {
            let steps: i8 = (octave as i8) * 12;
            return match note.step(steps) {
                Ok(note) => Some(note),
                Err(_) => None,
            };
        }
        return None;
    }
}
