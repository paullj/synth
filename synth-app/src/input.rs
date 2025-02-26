#[derive(Debug)]
pub enum InputEvent {
    KeyDown(Keycode),
    KeyUp(Keycode),
    IncrementEncoder(Encoder),
    DecrementEncoder(Encoder),
}

#[derive(Debug)]
pub enum Encoder {
    A,
    B,
    C,
    D,
}

#[derive(Debug)]
pub enum Keycode {
    A,
    B,
    X,
    Y,
}

pub trait Input {
    fn poll_input(&mut self) -> Option<Vec<InputEvent>>;
}
