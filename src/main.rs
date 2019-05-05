extern crate clap;
extern crate rodio;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::thread;
use std::time;

use clap::{App, Arg, Values};
use rodio::decoder::Decoder;
use rodio::source::Repeat;
use rodio::{Device, Sink, Source};

/// Prints out general error message and exits the program "gracefully"
fn exit_gracefully<E: std::fmt::Debug, T: std::fmt::Debug>(msg: E) -> T {
    panic!("{:?}", msg);
}

/// Open file location and return File structure
fn open_file(file_loc: &str) -> File {
    return File::open(&file_loc).unwrap_or_else(|msg| {
        exit_gracefully(format!(
            "Problem opening file: {} -- Error: {}",
            file_loc, msg
        ))
    });
}

/// Play mp3 file at file location
fn play_music_file(file_loc: String) {
    let device: Device = rodio::default_output_device().unwrap();
    let file: File = open_file(&file_loc);
    let sink: Sink = Sink::new(&device);
    let source: Repeat<Decoder<BufReader<File>>> = rodio::Decoder::new(BufReader::new(file))
        .unwrap_or_else(|msg| panic!("Bad file: {} -- Error: {}", &file_loc, msg))
        .repeat_infinite();

    sink.append(source);
    sink.play();
    sink.detach();
}

/// Play music files at given location(s)
fn play_music(mp3s_loc: Values) {
    mp3s_loc.to_owned().for_each(|mp3_loc| {
        play_music_file(mp3_loc.to_owned());
    });
}

fn main() {
    let matches = App::new("pomodoro_clock_rust")
        .version("0.1.0")
        .author("Paul Naranja")
        .about("A CLI pomodoro clock")
        .arg(
            Arg::with_name("length_in_secs")
                .help("Length of pomodoro clock in secs")
                .short("l")
                .default_value("60")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mp3s")
                .help("The location of the mp3s to play")
                .short("m")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mp3s_loc: Values = matches.values_of("mp3s").unwrap().to_owned();
    let length_in_sec_str = matches.value_of("length_in_secs").unwrap();
    let length_in_secs = u64::from_str(length_in_sec_str).unwrap_or(60);

    play_music(mp3s_loc);

    println!("Playing sound(s)");
    thread::sleep(time::Duration::from_secs(length_in_secs));
    println!("!!!!FINISHED!!!!")
}

#[cfg(test)]
mod tests {
    use crate::play_music_file;

    #[test]
    #[should_panic]
    /// This assumes the test is running from the root directory where Cargo.toml is located
    fn play_music_test_bad_file() {
        let a = String::from("Cargo.toml");
        play_music_file(a);
    }

    #[test]
    #[should_panic]
    fn play_music_test_fake_file() {
        let a = String::from("A_Fake_File");
        play_music_file(a);
    }
}
