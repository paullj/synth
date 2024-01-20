use crate::midi::midi_to_bytes;
use anyhow::bail;
use crossbeam::{atomic::AtomicCell, queue::SegQueue};
use midir::{Ignore, MidiInput, MidiInputPort};
use std::{sync::Arc, thread::JoinHandle};
use wmidi::MidiMessage;

/// Starts a thread that reads MIDI messages from the given port and pushes them into a queue.
pub fn start_midi_input_thread(
    input: MidiInput,
    input_port: MidiInputPort,
    midi_messages: Arc<SegQueue<Vec<u8>>>,
    quit: Arc<AtomicCell<bool>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let midi_messages_clone = midi_messages.clone();
        let _conn_in = input
            .connect(
                &input_port,
                "midir-read-input",
                move |_, message: &[u8], _| midi_messages.push(message.to_vec()),
                (),
            )
            .unwrap();
        while !quit.load() {}
        midi_messages_clone.push(midi_to_bytes(MidiMessage::Reset).to_vec());
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
        println!("Chose MIDI device '{device_name}'");
        Ok(input_ports[0].clone())
    }
}
