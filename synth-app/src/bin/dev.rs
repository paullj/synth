use embedded_graphics::{geometry::Size, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use midir::{os::unix::VirtualOutput, MidiOutput, MidiOutputConnection};
use synth_app::app::{App, Event};
use synth_app::midi::midi_to_bytes;
use wmidi::{Channel, MidiMessage, Note, Velocity};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 160;
const SCALE: u32 = 2;
const MAX_FPS: u32 = 60;

/// Starts a display in the main thread and sends MIDI messages to a virtual output in a separate thread.
fn main() {
    let midi_con = start_midi_output();
    let mut app = App::new();
    app.run();

    start_emulator(app, midi_con);
}

fn start_midi_output() -> MidiOutputConnection {
    let midi_out: MidiOutput = match MidiOutput::new("midir sending output") {
        Ok(m) => m,
        Err(e) => {
            panic!("Failed to create MIDI output: {}", e);
        }
    };
    // let out_port = get_first_midi_device(&mut midi_out).unwrap();
    let conn_out = match midi_out.create_virtual("Emulated MIDI Out") {
        Ok(conn_out) => conn_out,
        Err(e) => {
            panic!("Failed to create MIDI output connection: {}", e);
        }
    };
    println!("Created virtual MIDI output poirt");
    conn_out
}

fn start_emulator(mut app: App, mut midi_con: MidiOutputConnection) {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new()
        .scale(SCALE)
        .max_fps(MAX_FPS)
        .build();
    let mut window = Window::new("simulator", &output_settings);

    'running: loop {
        display.clear(Rgb565::BLACK).unwrap();
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    app.next(Event::Quit);
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
    }
    midi_con.close();
}

enum KeyPress {
    Down,
    Up,
}

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
        None => match keycode {
            Keycode::Q => None,
            Keycode::W => None,
            Keycode::R => None,
            Keycode::T => None,
            Keycode::Y => None,
            Keycode::U => None,
            Keycode::O => None,
            Keycode::P => None,
            Keycode::Num1 => None,
            _ => None,
        },
    }
}

trait KeycodeNote {
    fn value(self) -> Option<Note>;
}

impl KeycodeNote for Keycode {
    fn value(self) -> Option<Note> {
        match self {
            Keycode::Z => Some(Note::C4),
            Keycode::S => Some(Note::Db4),
            Keycode::X => Some(Note::D4),
            Keycode::D => Some(Note::Eb4),
            Keycode::C => Some(Note::E4),
            Keycode::V => Some(Note::F4),
            Keycode::G => Some(Note::Gb4),
            Keycode::B => Some(Note::G4),
            Keycode::H => Some(Note::Ab4),
            Keycode::N => Some(Note::A4),
            Keycode::J => Some(Note::Bb4),
            Keycode::M => Some(Note::B4),
            _ => None,
        }
    }
}

fn keycode_to_note<'a>(keycode: Keycode, press: KeyPress) -> Option<MidiMessage<'a>> {
    let note = keycode.value();
    match note {
        Some(note) => match press {
            KeyPress::Down => Some(MidiMessage::NoteOn(Channel::Ch1, note, Velocity::MAX)),
            KeyPress::Up => Some(MidiMessage::NoteOff(Channel::Ch1, note, Velocity::MAX)),
        },
        None => None,
    }
}
