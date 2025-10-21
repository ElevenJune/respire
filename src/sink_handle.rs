use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{SineWave};
use std::time::Duration;
use std::io::Cursor;
use std::fs::File;
use std::io::BufReader;
//use cli_log::*;

pub struct SinkHandle {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
    path : String
}

impl SinkHandle {
    pub fn new() -> SinkHandle {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => (stream, handle),
            Err(_) => panic!("Failed to create stream"),
        };
        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(_) => panic!("Failed to create sink"),
        };
        SinkHandle {
            _stream: stream,
            _stream_handle: stream_handle,
            sink,
            path : String::new()
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.len()!=0
    }

    pub fn _is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn _set_source<'a>(&mut self, data: std::io::Cursor<&'static [u8]>, path: &str) {
        self.clear_if_playing();
        let source = Decoder::new(Cursor::new(data.into_inner()));
        match source {
            Ok(source) => self.sink.append(source.repeat_infinite()),
            Err(_) => println!("Failed to create source from {}",path),
        }
    }

    pub fn set_source_as_filepath(&mut self, path: &str){
        self.path = path.to_string();

        //let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
        //self.sink.append(source);
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn play_once(&mut self){
        let file: BufReader<File> = BufReader::new(File::open(&self.path).unwrap());
        let buffer = Decoder::new(file).unwrap();
        self.sink.append(buffer);
        if self.sink.is_paused() {self.play()}
    }

    pub fn _pause(&mut self) {
        self.sink.pause();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.sink.clear();
    }

    fn clear_if_playing(&mut self) {
        if self.is_playing() {
            self.stop();
        }
    }
}
