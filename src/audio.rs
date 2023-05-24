use rodio::Source;
use std::io::Cursor;
use std::io::Read;
use std::time::Duration;

struct DmxDecoder {
    data: Cursor<Vec<u8>>,
    sample_rate: u32,
}

impl DmxDecoder {
    pub fn new(data: Vec<u8>, sample_rate: u32) -> Self {
        Self {
            data: Cursor::new(data),
            sample_rate,
        }
    }
}

impl Iterator for DmxDecoder {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1];
        match self.data.read(&mut buf) {
            Ok(0) => None,                              // We've reached end of data
            Ok(_) => Some((buf[0] as i16 - 128) * 256), // convert u8 to i16
            Err(_) => None, // Handle read error, here we just stop the iterator
        }
    }
}

impl Source for DmxDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None // We don't know the current frame length; it's infinite until the data runs out
    }

    fn channels(&self) -> u16 {
        1 // DMX data is mono
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None // We don't know the total duration
    }
}

pub fn play_sound(lump: Vec<u8>) {
    let lump = lump.clone(); // clone the lump to move it into the new thread
    std::thread::spawn(move || {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let dmx_source = DmxDecoder::new(lump.to_vec(), 11025);
        stream_handle
            .play_raw(dmx_source.convert_samples::<f32>())
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}
