use std::{sync::Arc, usize};

use anyhow::{anyhow, bail};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};
use crossbeam::queue::SegQueue;
use midi_msg::{Channel, ChannelVoiceMsg, MidiMsg};

pub fn start_output_thread<const N: usize>(midi_msgs: Arc<SegQueue<MidiMsg>>) {
    std::thread::spawn(move || {
        // let mut player = StereoPlayer::<N>::new(program_table);
        // player.run_output(midi_msgs).unwrap();
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(anyhow!("failed to find a default output device"))?;
        let config = device.default_output_config()?;
        match config.sample_format() {
            SampleFormat::F32 => run_synth::<f32>(midi_msgs, device, config.into()),
            SampleFormat::I16 => run_synth::<i16>(midi_msgs, device, config.into()),
            SampleFormat::U16 => run_synth::<u16>(midi_msgs, device, config.into()),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
    });
}

fn run_synth<T: Sample + SizedSample + FromSample<f64>>(
    midi_msgs: Arc<SegQueue<MidiMsg>>,
    device: Device,
    config: StreamConfig,
) -> anyhow::Result<()> {
    let mut done = false;
    while !done {
        let stream = get_stream::<T>(&config, &device)?;
        stream.play()?;
        handle_messages(midi_msgs.clone());
    }
    Ok(())
}

// fn warm_up(midi_msgs: Arc<SegQueue<MidiMsg>>) {
//     for _ in 0..N {
//         midi_msgs.push(warm_up_msg(ChannelVoiceMsg::NoteOn {
//             note: 0,
//             velocity: 0,
//         }));
//         midi_msgs.push(warm_up_msg(ChannelVoiceMsg::NoteOff {
//             note: 0,
//             velocity: 0,
//         }));
//     }
// }

// fn warm_up_msg(msg: ChannelVoiceMsg) -> MidiMsg {
//     MidiMsg::ChannelVoice {
//         channel: Channel::Ch1,
//         msg,
//     }
// }

fn handle_messages(midi_msgs: Arc<SegQueue<MidiMsg>>) {
    loop {
        if let Some(msg) = midi_msgs.pop() {
            println!("Got message: {:?}", msg);
            // if let Some(relayed) = decode(msg.speaker, &msg) {
            // return relayed;
            // }
        }
    }
}

fn get_stream<T: Sample + SizedSample + FromSample<f64>>(
    config: &StreamConfig,
    device: &Device,
) -> anyhow::Result<Stream> {
    let sample_rate = config.sample_rate.0 as f64;
    // let mut sound = self.sound();
    // sound.reset();
    // sound.set_sample_rate(sample_rate);
    let mut next_value = move || (0.0, 0.0);
    // sound.get_stereo();
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error on stream: {err}");
    device
        .build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
        .or_else(|err| bail!("{err:?}"))
}

fn write_data<T: Sample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = Sample::from_sample::<f64>(sample.0);
        let right: T = Sample::from_sample::<f64>(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}
