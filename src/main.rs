extern crate clap;
extern crate rodio;

use std::fs::File;
use std::io::BufReader;
use std::process;
use std::str::FromStr;
use std::thread;
use std::time;

use clap::{App, Arg};
use rodio::{Device, Source};
use rodio::decoder::Decoder;

/// Prints out general error message and exits the program "gracefully"
fn exit_gracefully<E: std::fmt::Debug, T: std::fmt::Debug>(msg: E) -> T {
    eprintln!("{:?}", msg);
    process::exit(1);
}

fn play_music(file_loc: String) {
    let device: Device = rodio::default_output_device().unwrap();
    let file = File::open(&file_loc).unwrap_or_else(|msg| {
        exit_gracefully(format!("Problem opening file {}: {}", file_loc, msg))
    });

    let source: Decoder<BufReader<File>> = rodio::Decoder::new(BufReader::new(file)).unwrap();

    rodio::play_raw(&device, source.repeat_infinite().convert_samples());
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
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mp3")
                .help("The location of the mp3 to play")
                .short("m")
                .default_value("sounds/Thunder_HeavyRain_Wind.mp3")
                .takes_value(true),
        )
        .get_matches();

    let length_in_sec_str = matches.value_of("length_in_secs").unwrap();
    let length_in_secs = u64::from_str(length_in_sec_str).unwrap_or(60);
    let mp3_loc: String = matches.value_of("mp3").unwrap().to_owned();

    thread::spawn(|| play_music(mp3_loc));
    thread::sleep(time::Duration::from_secs(length_in_secs));
    println!("Playing ALARM!!!!")
}
