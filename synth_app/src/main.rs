mod constants;

use std::cell::RefCell;
use std::collections::HashMap;
use std::{thread, time::Duration};
use crossbeam_channel::bounded;
use cpal::{self, SizedSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::{signal, Sample, Signal};
use std::sync::mpsc;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn main() -> Result<(), std::convert::Infallible> {
    let (midi_tx, midi_rx) = bounded(0);
    let (exit_tx, exit_rx) = bounded(0);
    
    let audio_th = start_audio_thread(midi_rx, exit_rx);
    start_graphics_thread(midi_tx);

    exit_tx.send(true).expect("failed to send exit signal");
    audio_th.join().unwrap();

    Ok(())
}

#[derive(Debug)]
pub enum MidiEvent {
    NoteOn(u8),
    NoteOff(u8),
}
fn start_graphics_thread(midi_tx: crossbeam_channel::Sender<MidiEvent>)  {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        constants::display::WIDTH,
        constants::display::HEIGHT,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(4).max_fps(60).build();
    let mut window = Window::new("simulator", &output_settings);

    'running: loop {
        display.clear(constants::color::BLACK).unwrap();
        // DRAW
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }
}

#[derive(Debug)]
struct Note {
    frequency: f32,
}

pub struct AudioContext {
    sample_rate: f32,
    num_channels: usize,
    active_notes: RefCell<HashMap<u8, Note>>,
}

fn start_audio_thread(
    midi_rx: crossbeam_channel::Receiver<MidiEvent>,
    exit_rx: crossbeam_channel::Receiver<bool>,
) -> std::thread::JoinHandle<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Couldn't open output device");

    println!("Default output device: {}", device.name().unwrap());

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    println!("Supported audio format: {:?}", sample_format);
    println!("Supported audio config: {:?}", config);

    let audio_ctx = AudioContext {
        sample_rate: config.sample_rate.0 as f32,
        num_channels: config.channels as usize,
        active_notes: RefCell::new(HashMap::new()),
    };

    thread::spawn(move || match sample_format {
        cpal::SampleFormat::F32 => play_audio_stream::<f32>(&device, &config.into(), audio_ctx).unwrap(),
        cpal::SampleFormat::I16 => play_audio_stream::<i16>(&device, &config.into(), audio_ctx).unwrap(),
        cpal::SampleFormat::U16 => play_audio_stream::<u16>(&device, &config.into(), audio_ctx).unwrap(),
        _ => panic!("Unsupported format"),
    })
}


fn play_audio_stream<T: SizedSample + cpal::FromSample<f32>>(device: &cpal::Device, config: &cpal::StreamConfig, mut audio_ctx: AudioContext) -> Result<(), anyhow::Error> {
    // Create a new simulator display for local development
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        constants::display::WIDTH,
        constants::display::HEIGHT,
    ));

    // Create a new window
    let output_settings = OutputSettingsBuilder::new().scale(4).max_fps(60).build();
    let mut window = Window::new("simulator", &output_settings);

    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    // Create a signal chain to play back 1 second of each oscillator at A4.
    let hz = signal::rate(config.sample_rate.0 as f64).const_hz(440.0);
    let one_sec = config.sample_rate.0 as usize;
    let mut synth = hz
        .clone()
        .sine()
        .take(one_sec)
        .map(|s| s.to_sample::<f32>() * 0.2);

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = mpsc::sync_channel(1);

    // Create and run the stream.
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &complete_tx, &mut synth)
        },
        err_fn,
        Some(Duration::from_secs(1)),
    )?;
    stream.play()?;

    // Wait for playback to complete.
    complete_rx.recv().unwrap();
    stream.pause()?;

    'running: loop {
        display.clear(constants::color::BLACK).unwrap();
        // DRAW
        window.update(&display);

        for event in window.events() {
            
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}


fn write_data<T: SizedSample + cpal::FromSample<f32>>(
    output: &mut [T],
    channels: usize,
    complete_tx: &mpsc::SyncSender<()>,
    signal: &mut dyn Iterator<Item = f32>,
) where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let sample = match signal.next() {
            None => {
                complete_tx.try_send(()).ok();
                0.0
            }
            Some(sample) => sample,
        };
        let value: T = cpal::Sample::from_sample::<f32>(sample);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}