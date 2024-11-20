use rodio::{Decoder, OutputStream, Sink, Source};
use std::{fs::File, io::BufReader, path::Path};

use crate::terminate;

pub struct Player {
    sink: Sink,
    _output_stream: OutputStream,
}

impl Player {
    pub fn new() -> Self {
        let (_output_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink,
            _output_stream,
        }
    }

    pub fn play<P: AsRef<Path>>(&self, audio_path: P) {
        let source = get_source(audio_path);
        self.sink.append(source);
        self.sink.play();
    }

    pub fn play_forever<P: AsRef<Path>>(&self, audio_path: P) {
        let source = get_source(audio_path);
        self.sink.append(source.repeat_infinite());
        self.sink.play();
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    pub fn clear(&self) {
        self.sink.clear();
    }
}

fn get_source<P: AsRef<Path>>(audio_path: P) -> Decoder<BufReader<File>> {
    let audio = BufReader::new(File::open(&audio_path).unwrap_or_else(|_| {
        terminate(
            format!(
                "Could not open audio file `{}`",
                audio_path.as_ref().to_string_lossy()
            ),
            1,
        )
    }));
    Decoder::new(audio).unwrap_or_else(|_| {
        terminate(
            format!(
                "File format of `{}` is not supported",
                audio_path.as_ref().to_string_lossy()
            ),
            1,
        )
    })
}
