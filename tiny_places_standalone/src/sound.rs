use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{Source, Buffered};


pub enum Sound {
    Click = 0,
}


pub struct SoundPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
    sources: Vec <Buffered<Decoder<BufReader<File>>>>
}


impl SoundPlayer {

    pub fn new() -> SoundPlayer {

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("resources/sounds/click.wav").unwrap());
        // let file = BufReader::new(File::open("../tiny_places_client/resources/sfx/fireball_launch.wav").unwrap());

        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap().buffered();

        // stream must live as long as the sink
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Play the sound directly on the device
        // stream_handle.play_raw(source.clone().convert_samples());


        // sink.append(source);
        let mut sources = Vec::new();
        sources.push(source);

        SoundPlayer {
            _stream: stream,
            _stream_handle: stream_handle,
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
