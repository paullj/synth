use std::convert::TryFrom;

use wmidi::{FromBytesError, MidiMessage};

// Decoding messages from bytes.
pub fn bytes_to_midi<'a>(bytes: &'a [u8]) -> Result<MidiMessage<'a>, FromBytesError> {
    let message = MidiMessage::try_from(bytes);
    match message {
        Ok(message) => Ok(message),
        Err(e) => {
            println!("Error handling MIDI message: {}", e);
            Err(e)
        }
    }
}

// Encoding messages to bytes.
pub fn midi_to_bytes(message: MidiMessage<'_>) -> Vec<u8> {
    let mut bytes = vec![0u8; message.bytes_size()];
    message.copy_to_slice(bytes.as_mut_slice()).unwrap();
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use wmidi::{Channel, Note, Velocity};

    #[test]
    fn test_midi_to_bytes_note_on() {
        let message = MidiMessage::NoteOn(Channel::Ch1, Note::C3, Velocity::MAX);

        let buffer = midi_to_bytes(message);

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer[1], 0x90);
        assert_eq!(buffer[2], 0x30);
        assert_eq!(buffer[3], 0x7F);
    }

    #[test]
    fn test_handle_midi_message_valid() {
        let bytes = &[0x90, 0x30, 0x7F];
        let message = bytes_to_midi(bytes);
        assert!(message.is_ok());
        assert_eq!(
            message.unwrap(),
            MidiMessage::NoteOn(Channel::Ch1, Note::C3, Velocity::MAX)
        );
    }

    #[test]
    fn test_handle_midi_message_invalid() {
        let bytes = &[0x90, 0x3C];
        let message = bytes_to_midi(bytes);
        assert!(message.is_err());
    }

    #[test]
    fn test_handle_midi_message_empty() {
        let bytes = &[];
        let message = bytes_to_midi(bytes);
        assert!(message.is_err());
    }

    #[test]
    fn test_handle_unexpected_data_byte() {
        let bytes = &[0x0, 0x3C, 0x7F];
        let message = bytes_to_midi(bytes);
        assert!(message.is_err());
    }
}
