use clap::{Values};
use rodio::decoder::Decoder;
use rodio::source::Repeat;
use rodio::{Device, Sink, Source};
use std::fs::File;
use std::io::BufReader;

use crate::util;

/// Play mp3 file at file location
fn play_music_file(file_loc: String) {
    let device: Device = rodio::default_output_device().unwrap();
    let file: File = util::open_file(&file_loc);
    let sink: Sink = Sink::new(&device);
    let source: Repeat<Decoder<BufReader<File>>> = rodio::Decoder::new(BufReader::new(file))
        .unwrap_or_else(|msg| panic!("Bad file: {} -- Error: {}", &file_loc, msg))
        .repeat_infinite();

    sink.append(source);
    sink.play();
    sink.detach();
}

/// Play music files at given location(s)
pub fn play_music(mp3s_loc: Values) {
    mp3s_loc.to_owned().for_each(|mp3_loc| {
        play_music_file(mp3_loc.to_owned());
    });
}


#[cfg(test)]
mod tests {
    use crate::music;

    #[test]
    #[should_panic]
    /// This assumes the test is running from the root directory where Cargo.toml is located
    fn play_music_test_bad_file() {
        let a = String::from("Cargo.toml");
        music::play_music_file(a);
    }

    #[test]
    #[should_panic]
    fn play_music_test_fake_file() {
        let a = String::from("A_Fake_File");
        music::play_music_file(a);
    }
}
