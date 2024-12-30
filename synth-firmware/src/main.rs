#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod key_code;
mod key_map;
mod state;

use defmt::{error, info};
use defmt_rtt as _;
use panic_probe as _;

use key_code::KeyCode;
use state::State;

use analog_multiplexer::{DummyPin, Multiplexer};
use assign_resources::assign_resources;
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_rp::{adc, bind_interrupts, gpio, peripherals, usb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel;
use embassy_time::{Duration, Timer};
use embassy_usb::class::midi::MidiClass;
use embassy_usb::{Builder, Config};
use wmidi::{MidiMessage, U7};

assign_resources! {
    usb: UsbResources {
        usb: USB,
    }
    scan: ScanResources {
        adc: ADC,
        SELECT_2: PIN_1,
        SELECT_3: PIN_2,
        SELECT_1: PIN_3,
        SELECT_0: PIN_4,
        AM1_COM: PIN_26,
    }
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

const MAX_PACKET_SIZE: usize = 64;
const DEBOUNCE_THRESHOLD: u16 = 50;

enum Events {
    KeyPressed(KeyCode, u8),
    KeyReleased(KeyCode, u8),
}

static EVENT_CHANNEL: channel::Channel<CriticalSectionRawMutex, Events, 10> =
    channel::Channel::new();

#[embassy_executor::main]
async fn main(s: Spawner) -> () {
    info!("Initializing...");

    // Initialize the RP2040 peripherals
    let p = embassy_rp::init(Default::default());
    let r = split_resources! {p};

    let _ = s.spawn(scan(s, r.scan));
    let _ = s.spawn(usb_midi(s, r.usb));
}

#[embassy_executor::task]
async fn scan(_s: Spawner, r: ScanResources) -> () {
    let mut state = State::new();
    let mut adc = adc::Adc::new(r.adc, Irqs, Default::default());
    let mut adc_channel = adc::Channel::new_pin(r.AM1_COM, gpio::Pull::Down);
    let pins = (
        gpio::Output::new(r.SELECT_0, gpio::Level::Low),
        gpio::Output::new(r.SELECT_1, gpio::Level::Low),
        gpio::Output::new(r.SELECT_2, gpio::Level::Low),
        gpio::Output::new(r.SELECT_3, gpio::Level::Low),
        DummyPin {},
    );
    let mut multiplexer = Multiplexer::new(pins);
    loop {
        for key in key_map::LEFT_KEYS.iter() {
            multiplexer.set_channel(key.channel);

            let previous = state.positions[key.code as usize];
            let initial = match adc.read(&mut adc_channel).await {
                Ok(value) => value,
                Err(e) => {
                    error!("ADC read error: {:?}", e);
                    2000
                }
            };
            if initial.abs_diff(previous) > DEBOUNCE_THRESHOLD {
                Timer::after(Duration::from_millis(10)).await;
                let next = match adc.read(&mut adc_channel).await {
                    Ok(value) => value,
                    Err(e) => {
                        error!("ADC read error: {:?}", e);
                        2000
                    }
                };
                if next.abs_diff(previous) > DEBOUNCE_THRESHOLD {
                    let velocity = next.abs_diff(previous).clamp(0, 127) as u8;
                    if previous > next {
                        if !state.notes_on[key.code as usize] {
                            state.notes_on[key.code as usize] = true;
                            EVENT_CHANNEL
                                .send(Events::KeyPressed(key.code, velocity))
                                .await;
                        }
                    } else {
                        if state.notes_on[key.code as usize] {
                            state.notes_on[key.code as usize] = false;
                            EVENT_CHANNEL
                                .send(Events::KeyReleased(key.code, velocity))
                                .await;
                        }
                    }
                }
                state.positions[key.code as usize] = next;
            }
        }
    }
}

#[embassy_executor::task]
async fn usb_midi(_s: Spawner, r: UsbResources) -> () {
    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("paullj");
    config.product = Some("MIDI Keyboard");
    config.serial_number = Some("PICO");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; MAX_PACKET_SIZE];

    let driver = usb::Driver::new(r.usb, Irqs);
    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    // Create classes on the builder.
    let mut midi_class = MidiClass::new(&mut builder, 1, 1, 64);
    // Build the builder.
    let mut usb_device = builder.build();
    // Run the USB device.
    let usb_future = usb_device.run();
    // Use the Midi class.
    let midi_future = async {
        midi_class.wait_connection().await;

        let receiver = EVENT_CHANNEL.receiver();
        loop {
            let event = receiver.receive().await;
            match event {
                Events::KeyPressed(key, velocity) => {
                    if let Some(note) = key.to_note(3) {
                        let message = MidiMessage::NoteOn(
                            wmidi::Channel::Ch1,
                            note,
                            U7::from_u8_lossy(velocity),
                        );
                        let (buffer, n) = midi_to_bytes(message);
                        let _ = midi_class.write_packet(&buffer[..n]).await;
                    }
                }
                Events::KeyReleased(key, velocity) => {
                    if let Some(note) = key.to_note(3) {
                        let message = MidiMessage::NoteOff(
                            wmidi::Channel::Ch1,
                            note,
                            U7::from_u8_lossy(velocity),
                        );
                        let (buffer, n) = midi_to_bytes(message);
                        let _ = midi_class.write_packet(&buffer[..n]).await;
                    }
                }
            }
        }
    };

    join::join(usb_future, midi_future).await;
    usb_device.disable().await;
}

fn midi_to_bytes(message: wmidi::MidiMessage<'_>) -> ([u8; MAX_PACKET_SIZE], usize) {
    // For some reason the first byte has to be 0x8 or 0x9, otherwise the message is not sent.
    let mut buffer = [0x8; MAX_PACKET_SIZE];
    let n = message.bytes_size();
    if n > buffer.len() {
        panic!("MIDI message too large");
    }
    message.copy_to_slice(&mut buffer[1..n + 1]).unwrap();
    (buffer, n + 1)
}
