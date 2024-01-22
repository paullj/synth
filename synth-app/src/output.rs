use std::{collections::HashMap, sync::Arc, thread::JoinHandle, usize};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, StreamConfig,
};
use crossbeam::queue::SegQueue;
use fundsp::hacker::*;
use wmidi::Note;

use crate::midi::bytes_to_midi;

pub fn start_output_thread(midi_messages: Arc<SegQueue<Vec<u8>>>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut player = Player::<6>::new();
        match player.run(midi_messages) {
            Ok(_) => (),
            Err(err) => panic!("Error! {:?}", err),
        }
    })
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

struct Player<const N: usize> {
    notes: HashMap<Note, EventId>,
}

impl<const N: usize> Player<N> {
    fn new() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }

    fn run(&mut self, midi_messages: Arc<SegQueue<Vec<u8>>>) -> Result<(), anyhow::Error> {
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(device) => device,
            None => panic!("No output found!"),
        };
        let supported_config = device.default_output_config()?;
        let config = supported_config.config();

        println!("Output device: {:}", device.name()?);

        match supported_config.sample_format() {
            SampleFormat::F32 => self.run_synth::<f32>(midi_messages, device, config)?,
            SampleFormat::I16 => self.run_synth::<i16>(midi_messages, device, config)?,
            SampleFormat::U16 => self.run_synth::<u16>(midi_messages, device, config)?,
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        };
        Ok(())
    }

    fn run_synth<T: Sample + SizedSample + FromSample<f64>>(
        &mut self,
        midi_msgs: Arc<SegQueue<Vec<u8>>>,
        device: Device,
        config: StreamConfig,
    ) -> Result<(), anyhow::Error> {
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;

        let mut sequencer = Sequencer64::new(false, 1);
        let sequencer_backend = sequencer.backend();
        let mut network = Net64::wrap(Box::new(sequencer_backend));

        network = network >> pan(0.0);

        network.set_sample_rate(sample_rate);

        // Use block processing for maximum efficiency.
        let mut backend = BlockRateAdapter64::new(Box::new(network.backend()));

        let mut next_value = move || backend.get_stereo();
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )?;
        stream.play()?;
        self.handle_messages(midi_msgs, &mut sequencer);
        Ok(())
    }

    fn handle_messages(&mut self, midi_msgs: Arc<SegQueue<Vec<u8>>>, sequencer: &mut Sequencer64) {
        let state = SynthState::default();
        loop {
            if let Some(msg) = midi_msgs.pop() {
                match bytes_to_midi(&msg) {
                    Ok(message) => match message {
                        wmidi::MidiMessage::NoteOn(_, note, _) => {
                            let pitch_hz = note.to_freq_f64();
                            let v = state.vibrato_depth.value() * 0.003;
                            let pitch = lfo(move |t| {
                                pitch_hz
                                    * xerp11(
                                        1.0 / (1.0 + v),
                                        1.0 + v,
                                        sin_hz(6.0, t) + sin_hz(6.1, t),
                                    )
                            });

                            let waveform = Net64::wrap(Box::new(pitch >> saw() * 0.5));
                            let filter = Net64::wrap(Box::new(pass()));

                            let ads =
                                (var(&state.attack) | var(&state.decay) | var(&state.sustain))
                                    >> lfo_in(|t, adsr: &Frame<f64, U3>| {
                                        let a = adsr[0];
                                        let d = adsr[1];
                                        let s = adsr[2];
                                        if t < a {
                                            delerp(0.0, a, t)
                                        } else if t < a + d {
                                            lerp(1.0, s, delerp(a, a + d, t))
                                        } else {
                                            s
                                        }
                                    });

                            let unit = Box::new(waveform * ads >> filter * var(&state.level));

                            let event_id = sequencer.push_relative(
                                0.0,
                                f64::INFINITY,
                                Fade::Smooth,
                                0.0,
                                0.0,
                                unit,
                            );
                            self.notes.insert(note, event_id);
                        }
                        wmidi::MidiMessage::NoteOff(_, note, _) => {
                            if let Some(event_id) = self.notes.get(&note) {
                                sequencer.edit_relative(
                                    *event_id,
                                    state.release.value(),
                                    state.release.value(),
                                );
                            }
                        }
                        _ => (),
                    },
                    Err(err) => {
                        println!("Error decoding message: {}", err);
                        continue;
                    }
                };
            }
        }
    }
}

struct SynthState {
    // Page 1
    attack: Shared<f64>,
    decay: Shared<f64>,
    sustain: Shared<f64>,
    release: Shared<f64>,
    // Page 2
    control_1: Shared<f64>,
    control_2: Shared<f64>,
    control_3: Shared<f64>,
    control_4: Shared<f64>,
    // Page 3
    filter_cutoff: Shared<f64>,
    filter_resonance: Shared<f64>,
    filter_type: Shared<f64>,
    level: Shared<f64>,
    // Page 4
    vibrato_rate: Shared<f64>,
    vibrato_depth: Shared<f64>,
}

impl Default for SynthState {
    fn default() -> Self {
        Self {
            attack: shared(0.4),
            decay: shared(0.1),
            sustain: shared(0.5),
            release: shared(2.0),
            control_1: shared(0.5),
            control_2: shared(0.5),
            control_3: shared(0.5),
            control_4: shared(0.5),
            filter_cutoff: shared(0.1),
            filter_resonance: shared(0.1),
            filter_type: shared(0.1),
            level: shared(1.0),
            vibrato_depth: shared(0.0),
            vibrato_rate: shared(0.0),
        }
    }
}
