use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use std::time;

use rodio::Source;

fn handle_args(args: &Vec<String>) {
    if args.len() == 1 {
        println!("USAGE: pomodoro_clock_rust <length_in_secs> <mp3>");
        exit(1);
    }
}

fn play_music(file_loc: String) {
    let device = rodio::default_output_device().unwrap();

    let file = File::open(file_loc).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

}

fn main() {
    let args: Vec<String> = args().collect();
    handle_args(&args);

    let length_in_secs = u64::from_str(&args[1]).unwrap();
    let mp3_loc: String = args.get(2).unwrap().to_owned();

    thread::spawn(|| play_music(mp3_loc));
    thread::sleep(time::Duration::from_secs(length_in_secs));
    println!("Playing ALARM!!!!")
}
