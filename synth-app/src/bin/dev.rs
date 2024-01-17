use embedded_graphics::{geometry::Size, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use midi_msg::MidiMsg;
use midir::{os::unix::VirtualOutput, MidiOutput, MidiOutputConnection};

use synth_app::app::App;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 160;
const SCALE: u32 = 2;
const MAX_FPS: u32 = 60;

/// Starts a display in the main thread and sends MIDI messages to a virtual output in a separate thread.
fn main() {
    let midi_con = start_midi_output();
    std::thread::spawn(move || {
        let mut app = App::new();

        app.run();
    });
    start_emulator(midi_con);
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

fn start_emulator(mut midi_con: MidiOutputConnection) {
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
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if !repeat {
                        match keycode_to_midi(keycode, KeyPress::Down) {
                            Some(msg) => {
                                midi_con.send(&msg.to_midi()).unwrap();
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
                                midi_con.send(&msg.to_midi()).unwrap();
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

fn keycode_to_midi(keycode: Keycode, press: KeyPress) -> Option<MidiMsg> {
    let note_message = keycode_to_note(
        keycode,
        match press {
            KeyPress::Down => KeyPress::Down,
            KeyPress::Up => KeyPress::Up,
        },
    );
    match note_message {
        Some(msg) => Some(MidiMsg::ChannelVoice {
            channel: midi_msg::Channel::Ch1,
            msg: msg,
        }),
        None => match keycode {
            Keycode::Q => Some(MidiMsg::SystemRealTime {
                msg: midi_msg::SystemRealTimeMsg::Start,
            }),
            Keycode::W => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::R => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::T => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::Y => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::U => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::O => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::P => Some(MidiMsg::SystemExclusive {
                msg: midi_msg::SystemExclusiveMsg::NonCommercial { data: vec![] },
            }),
            Keycode::Num1 => Some(MidiMsg::ChannelVoice {
                channel: midi_msg::Channel::Ch1,
                msg: midi_msg::ChannelVoiceMsg::ControlChange {
                    control: midi_msg::ControlChange::GeneralPurpose1(1),
                },
            }),
            _ => None,
        },
    }
}

trait KeycodeNote {
    fn value(self) -> Option<u8>;
}

impl KeycodeNote for Keycode {
    fn value(self) -> Option<u8> {
        match self {
            Keycode::Z => Some(69),
            Keycode::S => Some(70),
            Keycode::X => Some(71),
            Keycode::D => Some(72),
            Keycode::C => Some(73),
            Keycode::V => Some(74),
            Keycode::G => Some(75),
            Keycode::B => Some(76),
            Keycode::H => Some(77),
            Keycode::N => Some(78),
            Keycode::J => Some(79),
            Keycode::M => Some(80),
            _ => None,
        }
    }
}

fn keycode_to_note(keycode: Keycode, press: KeyPress) -> Option<midi_msg::ChannelVoiceMsg> {
    let note = keycode.value();
    match note {
        Some(note) => match press {
            KeyPress::Down => Some(midi_msg::ChannelVoiceMsg::NoteOn {
                note,
                velocity: 127,
            }),
            KeyPress::Up => Some(midi_msg::ChannelVoiceMsg::NoteOff {
                note,
                velocity: 127,
            }),
        },
        None => None,
    }
}
