use crate::key_code::KeyCode;

const LEFT_NUM_CHANNELS: usize = 15;

pub struct KeyMap {
    pub channel: u8,
    pub code: KeyCode,
}

impl KeyMap {
    const fn new(channel: u8, key: KeyCode) -> Self {
        Self { channel, code: key }
    }
}

pub const LEFT_KEYS: [KeyMap; LEFT_NUM_CHANNELS] = [
    KeyMap::new(6, KeyCode::SHIFT),
    KeyMap::new(7, KeyCode::UP),
    KeyMap::new(8, KeyCode::DOWN),
    KeyMap::new(9, KeyCode::C1),
    KeyMap::new(5, KeyCode::CSharp1),
    KeyMap::new(10, KeyCode::D1),
    KeyMap::new(4, KeyCode::DSharp1),
    KeyMap::new(11, KeyCode::E1),
    KeyMap::new(12, KeyCode::F1),
    KeyMap::new(2, KeyCode::FSharp1),
    KeyMap::new(13, KeyCode::G1),
    KeyMap::new(1, KeyCode::GSharp1),
    KeyMap::new(14, KeyCode::A1),
    KeyMap::new(0, KeyCode::ASharp1),
    KeyMap::new(15, KeyCode::B1),
];
