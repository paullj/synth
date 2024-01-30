use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use midir::{os::unix::VirtualOutput, MidiOutput, MidiOutputConnection};
use synth_app::app::App;
use synth_app::midi::midi_to_bytes;
use wmidi::{Channel, ControlFunction, MidiMessage, Note, Velocity, U7};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 160;
const SCALE: u32 = 2;
const MAX_FPS: u32 = 60;

/// Starts a display in the main thread and sends MIDI messages to a virtual output in a separate thread.
fn main() {
    // Set up MIDI output
    let mut midi_con = start_midi_output();

    let help_text = "    S   D       G   H   J\n\
    |  |#| |#|  |  |#| |#| |#|  |  > Play   Q\n\
    |  |#| |#|  |  |#| |#| |#|  |  ● Record W\n\
    |  |#| |#|  |  |#| |#| |#|  |  ≡ Menu   O\n\
    |   |   |   |   |   |   |   |  v Select P\n\
    |___|___|___|___|___|___|___|\n\
      Z   X   C   V   B   N   M";

    println!("{}", help_text);

    // Create a shared display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new()
        .scale(SCALE)
        .max_fps(MAX_FPS)
        .build();

    // Set up app
    let mut window = Window::new("simulator", &output_settings);
    window.update(&display);

    let mut app = App::new();

    'running: loop {
        let frame_start = std::time::Instant::now();

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    break 'running;
                }
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if !repeat {
                        match keycode_to_midi(keycode, KeyPress::Down) {
                            Some(msg) => {
                                midi_con.send(midi_to_bytes(msg).as_slice()).unwrap();
                            }
                            None => {}
                        }
                    }
                }
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if !repeat {
                        match keycode_to_midi(keycode, KeyPress::Up) {
                            Some(msg) => {
                                midi_con.send(midi_to_bytes(msg).as_slice()).unwrap();
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }
      
        app.update();
        match app.draw(&mut display) {
            Ok(_) => {}
            Err(err) => println!("Error drawing: {}", err),
        }

        window.update(&display);
    }
    midi_con.close();
}

fn start_midi_output() -> MidiOutputConnection {
    let midi_out: MidiOutput = match MidiOutput::new("midir sending output") {
        Ok(m) => m,
        Err(e) => {
            panic!("Failed to create MIDI output: {}", e);
        }
    };
    let conn_out = match midi_out.create_virtual("Emulated MIDI Out") {
        Ok(conn_out) => conn_out,
        Err(e) => {
            panic!("Failed to create MIDI output connection: {}", e);
        }
    };
    println!("Created virtual MIDI output port for dev");
    conn_out
}

#[derive(Debug, PartialEq)]
enum KeyPress {
    Down,
    Up,
}

const RECORD: &[u8] = &[0x02, 0x02, 0x02];
const MENU: &[u8] = &[0x03, 0x03, 0x03];
const SELECT: &[u8] = &[0x04, 0x04, 0x04];

fn keycode_to_midi<'a>(keycode: Keycode, press: KeyPress) -> Option<MidiMessage<'a>> {
    let note_message = keycode_to_note(
        keycode,
        match press {
            KeyPress::Down => KeyPress::Down,
            KeyPress::Up => KeyPress::Up,
        },
    );
    match note_message {
        Some(msg) => Some(msg),
        None => {
            if press == KeyPress::Down {
                match keycode {
                    Keycode::Q => Some(MidiMessage::Start),
                    Keycode::W => Some(MidiMessage::SysEx(U7::try_from_bytes(RECORD).unwrap())),
                    Keycode::R => None,
                    Keycode::T => None,
                    Keycode::Y => None,
                    Keycode::U => None,
                    Keycode::O => Some(MidiMessage::SysEx(U7::try_from_bytes(MENU).unwrap())),
                    Keycode::P => Some(MidiMessage::SysEx(U7::try_from_bytes(SELECT).unwrap())),
                    // TODO: Probably will need to use a different MIDI message for these
                    Keycode::Num1 => Some(MidiMessage::ControlChange(
                        Channel::Ch1,
                        ControlFunction::DATA_DECREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num2 => Some(MidiMessage::ControlChange(
                        Channel::Ch1,
                        ControlFunction::DATA_INCREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num3 => Some(MidiMessage::ControlChange(
                        Channel::Ch2,
                        ControlFunction::DATA_DECREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num4 => Some(MidiMessage::ControlChange(
                        Channel::Ch2,
                        ControlFunction::DATA_INCREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num5 => Some(MidiMessage::ControlChange(
                        Channel::Ch3,
                        ControlFunction::DATA_DECREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num6 => Some(MidiMessage::ControlChange(
                        Channel::Ch3,
                        ControlFunction::DATA_INCREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num7 => Some(MidiMessage::ControlChange(
                        Channel::Ch4,
                        ControlFunction::DATA_DECREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num8 => Some(MidiMessage::ControlChange(
                        Channel::Ch4,
                        ControlFunction::DATA_INCREMENT,
                        U7::MAX,
                    )),
                    Keycode::Num9 => None,
                    Keycode::Num0 => None,
                    Keycode::Minus => Some(MidiMessage::ControlChange(
                        Channel::Ch1,
                        ControlFunction::DATA_DECREMENT,
                        U7::MAX,
                    )),
                    Keycode::Equals => Some(MidiMessage::ControlChange(
                        Channel::Ch1,
                        ControlFunction::DATA_INCREMENT,
                        U7::MAX,
                    )),
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}

trait KeycodeNote {
    fn value(self, octave: u8) -> Option<Note>;
}

impl KeycodeNote for Keycode {
    fn value(self, octave: u8) -> Option<Note> {
        if octave > 10 {
            return None;
        }

        match self {
            Keycode::Z => Some(Note::from_u8_lossy(0 + (octave * 12))),
            Keycode::S => Some(Note::from_u8_lossy(1 + (octave * 12))),
            Keycode::X => Some(Note::from_u8_lossy(2 + (octave * 12))),
            Keycode::D => Some(Note::from_u8_lossy(3 + (octave * 12))),
            Keycode::C => Some(Note::from_u8_lossy(4 + (octave * 12))),
            Keycode::V => Some(Note::from_u8_lossy(5 + (octave * 12))),
            Keycode::G => Some(Note::from_u8_lossy(6 + (octave * 12))),
            Keycode::B => Some(Note::from_u8_lossy(7 + (octave * 12))),
            Keycode::H => Some(Note::from_u8_lossy(8 + (octave * 12))),
            Keycode::N => Some(Note::from_u8_lossy(9 + (octave * 12))),
            Keycode::J => Some(Note::from_u8_lossy(10 + (octave * 12))),
            Keycode::M => Some(Note::from_u8_lossy(11 + (octave * 12))),
            _ => None,
        }
    }
}

fn keycode_to_note<'a>(keycode: Keycode, press: KeyPress) -> Option<MidiMessage<'a>> {
    let note = keycode.value(4);
    match note {
        Some(note) => match press {
            KeyPress::Down => Some(MidiMessage::NoteOn(Channel::Ch1, note, Velocity::MAX)),
            KeyPress::Up => Some(MidiMessage::NoteOff(Channel::Ch1, note, Velocity::MAX)),
        },
        None => None,
    }
}
