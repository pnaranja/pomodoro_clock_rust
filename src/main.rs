use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time;

use rodio::Source;

fn play_music() {
    let file_loc = "sounds/Thunder_HeavyRain_Wind.mp3";
    let device = rodio::default_output_device().unwrap();

    let file = File::open(file_loc).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

}

fn main() {
    thread::spawn(play_music);
    thread::sleep(time::Duration::from_secs(5));
    println!("Playing ALARM!!!!")
}
