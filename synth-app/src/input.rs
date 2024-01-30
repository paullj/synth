use crate::{
    app::{ActionMessage, Direction, EngineMessage, StateParameter},
    midi::bytes_to_midi,
};
use anyhow::bail;
use crossbeam::{atomic::AtomicCell, queue::SegQueue};
use midir::{Ignore, MidiInput, MidiInputPort};
use std::{sync::Arc, thread::JoinHandle};
use wmidi::{Channel, ControlFunction, MidiMessage, U7};

/// Starts a thread that reads MIDI messages from the given port and pushes them into a queue.
pub fn start_midi_input_thread(
    input: MidiInput,
    input_port: MidiInputPort,
    engine_messages: Arc<SegQueue<EngineMessage>>,
    action_messages: Arc<SegQueue<ActionMessage>>,
    quit: Arc<AtomicCell<bool>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let action_messages_clone = action_messages.clone();
        let _conn_in = input
            .connect(
                &input_port,
                "synth-app-midi-input",
                move |_, bytes: &[u8], _| match bytes_to_midi(bytes) {
                    Ok(message) => match message {
                        MidiMessage::NoteOff(_, note, _) => {
                            engine_messages.push(EngineMessage::NoteOff(note));
                        }
                        MidiMessage::NoteOn(_, note, _) => {
                            engine_messages.push(EngineMessage::NoteOn(note));
                        }
                        MidiMessage::ControlChange(
                            Channel::Ch1,
                            ControlFunction::DATA_INCREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Increment,
                            StateParameter::Attack,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch1,
                            ControlFunction::DATA_DECREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Decrement,
                            StateParameter::Attack,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch2,
                            ControlFunction::DATA_INCREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Increment,
                            StateParameter::Decay,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch2,
                            ControlFunction::DATA_DECREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Decrement,
                            StateParameter::Decay,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch3,
                            ControlFunction::DATA_INCREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Increment,
                            StateParameter::Sustain,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch3,
                            ControlFunction::DATA_DECREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Decrement,
                            StateParameter::Sustain,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch4,
                            ControlFunction::DATA_INCREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Increment,
                            StateParameter::Release,
                        )),
                        MidiMessage::ControlChange(
                            Channel::Ch4,
                            ControlFunction::DATA_DECREMENT,
                            _,
                        ) => engine_messages.push(EngineMessage::ChangeParameter(
                            Direction::Decrement,
                            StateParameter::Release,
                        )),
                        MidiMessage::Start => action_messages.push(ActionMessage::A),
                        MidiMessage::SysEx(bytes) => {
                            // TODO: A better way to match custom SysEx messages
                            if bytes == U7::try_from_bytes(&[0x02, 0x02, 0x02]).unwrap() {
                                action_messages.push(ActionMessage::B);
                            } else if bytes == U7::try_from_bytes(&[0x03, 0x03, 0x03]).unwrap() {
                                action_messages.push(ActionMessage::X);
                            } else if bytes == U7::try_from_bytes(&[0x04, 0x04, 0x04]).unwrap() {
                                action_messages.push(ActionMessage::Y);
                            }
                        }
                        _ => (),
                    },
                    Err(err) => println!("MIDI From Bytes Error: {}", err),
                },
                (),
            )
            .unwrap();
        while !quit.load() {}
        action_messages_clone.push(ActionMessage::Quit);
        quit.store(false);
    })
}

/// Returns a handle to the first MIDI device detected.
pub fn get_first_midi_device(midi_input: &mut MidiInput) -> anyhow::Result<MidiInputPort> {
    midi_input.ignore(Ignore::None);
    let input_ports = midi_input.ports();
    if input_ports.len() == 0 {
        bail!("No MIDI devices attached")
    } else {
        let device_name = midi_input.port_name(&input_ports[0])?;
        println!("Input MIDI device: {}", device_name);
        Ok(input_ports[0].clone())
    }
}
