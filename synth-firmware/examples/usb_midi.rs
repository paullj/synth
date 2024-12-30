#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_time::{Duration, Timer};
use embassy_usb::class::midi::MidiClass;
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};
use wmidi::{Channel, MidiMessage, Note, Velocity};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const MAX_PACKET_SIZE: usize = 64;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    defmt::info!("Initialized.");
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Paul Lavender-Jones");
    config.product = Some("MIDI Keyboard");
    config.serial_number = Some("PICO0001");
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
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; MAX_PACKET_SIZE];

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    // Create classes on the builder.
    let mut class = MidiClass::new(&mut builder, 1, 1, 64);

    // The `MidiClass` can be split into `Sender` and `Receiver`, to be used in separate tasks.
    // let (sender, receiver) = class.split();

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    // Use the Midi class!
    let midi_fut = async {
        class.wait_connection().await;
        loop {
            defmt::info!("Connected");
            let _ = midi_echo(&mut class, &mut led).await;
            defmt::info!("Disconnected");
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, midi_fut).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
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

// Decoding messages from bytes.
fn handle_midi_message(bytes: &[u8]) -> Result<MidiMessage, wmidi::FromBytesError> {
    match wmidi::MidiMessage::try_from(bytes) {
        Ok(message) => Ok(message),
        Err(e) => Err(e),
    }
}

async fn midi_echo<'d, T: Instance + 'd>(
    class: &mut MidiClass<'d, Driver<'d, T>>,
    led: &mut Output<'_>,
) -> Result<(), Disconnected> {
    loop {
        led.set_high();
        defmt::info!("Sending MIDI message: NoteOn");
        let message = MidiMessage::NoteOn(Channel::Ch1, Note::C4, Velocity::MAX);
        let (buffer, n) = midi_to_bytes(message);
        let data = &buffer[0..n];
        handle_midi_message(&data[1..]).unwrap();
        class.write_packet(data).await?;
        Timer::after(Duration::from_millis(1000)).await;

        led.set_low();
        defmt::info!("Sending MIDI message: NoteOff");
        let message = MidiMessage::NoteOff(Channel::Ch1, Note::C4, Velocity::MAX);
        let (buffer, n) = midi_to_bytes(message);
        let data = &buffer[0..n];

        class.write_packet(data).await?;
        Timer::after(Duration::from_millis(3000)).await;
    }
}
