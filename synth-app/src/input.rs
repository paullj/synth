use anyhow::bail;
use crossbeam::atomic::AtomicCell;
use crossbeam::queue::SegQueue;
use midi_msg::{MidiMsg, SystemRealTimeMsg};
use midir::{Ignore, MidiInput, MidiInputPort};
use std::sync::Arc;

/// Starts a thread that reads MIDI messages from the given port and pushes them into a queue.
pub fn start_input_thread(
    midi_msgs: Arc<SegQueue<MidiMsg>>,
    midi_in: MidiInput,
    in_port: MidiInputPort,
    quit: Arc<AtomicCell<bool>>,
) {
    std::thread::spawn(move || {
        let midi_msgs_copy = midi_msgs.clone();
        let _conn_in = midi_in
            .connect(
                &in_port,
                "midir-read-input",
                move |_stamp, message, _| {
                    let (msg, _len) = MidiMsg::from_midi(&message).unwrap();
                    // println!("Received MIDI message: {:?}", msg);
                    midi_msgs.push(msg);
                },
                (),
            )
            .unwrap();
        while !quit.load() {}
        midi_msgs_copy.push(MidiMsg::SystemRealTime {
            msg: SystemRealTimeMsg::SystemReset,
        });
        quit.store(false);
    });
}

/// Returns a handle to the first MIDI device detected.
pub fn get_first_midi_device(midi_in: &mut MidiInput) -> anyhow::Result<MidiInputPort> {
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    if in_ports.len() == 0 {
        bail!("No MIDI devices attached")
    } else {
        let device_name = midi_in.port_name(&in_ports[0])?;
        println!("Chose MIDI device '{device_name}'");
        Ok(in_ports[0].clone())
    }
}
