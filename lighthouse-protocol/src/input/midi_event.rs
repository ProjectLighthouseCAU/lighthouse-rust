use serde::{Deserialize, Serialize};

/// A MIDI message event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MIDIEvent {
    /// The binary MIDI message.
    /// 
    /// The first byte is a status byte (first/most significant bit = 1), the
    /// remaining bytes are data bytes (first/most significant bit = 0).
    /// 
    /// To give a simple example, pressing C5 on a MIDI keyboard would generate the
    /// following message:
    /// 
    /// ```plaintext
    ///     [0x90,     0x48,     0x64]
    ///      Ch.1    Note 72   Velocity 100
    ///     NoteOn   i.e. C5
    /// ```
    /// 
    /// The note values can be looked up online:
    /// 
    /// - https://www.phys.unsw.edu.au/jw/notes.html
    /// 
    /// Same goes for a full description of the packet structure:
    /// 
    /// - https://www.w3.org/TR/webmidi/#terminology
    /// - http://www.opensound.com/pguide/midi/midi5.html
    /// - https://www.songstuff.com/recording/article/midi-message-format/
    data: Vec<u8>,
}
