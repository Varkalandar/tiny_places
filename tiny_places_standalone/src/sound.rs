use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{Source, Buffered};

pub enum Sound {
    Click = 0,
    Test = 1,
}


pub struct SoundPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    sources: Vec <Buffered<Decoder<BufReader<File>>>>
}


impl SoundPlayer {

    pub fn new() -> SoundPlayer {

        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("../tiny_places_client/resources/sfx/hard_click.wav").unwrap());

        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap().buffered();

        // Play the sound directly on the device
        // stream_handle.play_raw(source.convert_samples());

        // stream must live as long as the sink
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // sink.append(source);
        let mut sources = Vec::new();
        sources.push(source);

        SoundPlayer {
            stream,
            stream_handle,
            sink,
            sources,
        }
    }

    pub fn play_sound(&self, id: Sound) {
        let index = id as usize;
        println!("Playing sound {}", index);
        self.sink.append(self.sources[index].clone());
    }
}
