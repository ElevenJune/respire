use rodio::source::{Source, Buffered};
use rodio::{Decoder, OutputStream,  Sink};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

type ClonableSource = Buffered<Decoder<BufReader<File>>>;

pub struct SinkHandle {
    _stream_handle: OutputStream,
    sink: Sink,
    source: Option<ClonableSource>,
}

impl SinkHandle {
    pub fn new() -> SinkHandle {
        let stream_handle = match rodio::OutputStreamBuilder::open_default_stream() {
            Ok(stream_handle) => stream_handle,
            Err(_) => panic!("Failed to create stream"),
        };
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        SinkHandle {
            _stream_handle: stream_handle,
            sink,
            source:None
        }
    }

    pub fn _is_playing(&self) -> bool {
        self.sink.len()!=0
    }

    pub fn _is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn load_file_source<P: AsRef<Path>>(&mut self, path: P) {
        let path_ref = path.as_ref();
        
        match File::open(path_ref) {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                
                // Decoder::new now handles the decoding and conversion to f32 samples internally.
                match Decoder::new(buf_reader) {
                    Ok(decoder) => {
                        // The key: call .buffered() to read the data into memory,
                        // making the resulting source clonable.
                        self.source = Some(decoder.buffered());
                        //println!("Successfully loaded and buffered source from: {}", path_ref.display());
                    },
                    Err(e) => eprintln!("Failed to decode file {}: {}", path_ref.display(), e),
                }
            },
            Err(e) => eprintln!("Failed to open file {}: {}", path_ref.display(), e),
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn play_once(&mut self){
        // 1. Get a mutable reference to the stored source
        if let Some(ref source) = self.source {
            // 2. Clone the buffered source (cheap memory copy, no file I/O or re-decoding)
            let cloned_source = source.clone();
            
            // 3. Append the cloned source to the Sink
            self.sink.append(cloned_source);
            
            // 4. Ensure the Sink is playing
            if self.sink.is_paused() {
                self.play()
            }
        } else {
            eprintln!("Error: Source not loaded. Call load_file_source first.");
        }
    }

    pub fn _pause(&mut self) {
        self.sink.pause();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.sink.clear();
    }

    fn _clear_if_playing(&mut self) {
        if self._is_playing() {
            self.stop();
        }
    }
}
