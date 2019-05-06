extern crate clap;
extern crate rodio;

use std::str::FromStr;
use std::thread;
use std::time;

use clap::Values;

mod music;
mod util;

fn main() {
    let matches = util::parse_args();
    let mp3s_loc: Values = matches.values_of("mp3s").unwrap().to_owned();
    let length_in_sec_str = matches.value_of("length_in_secs").unwrap();
    let length_in_secs = u64::from_str(length_in_sec_str).unwrap_or(60);

    music::play_music(mp3s_loc);

    println!("Playing sound(s)");
    thread::sleep(time::Duration::from_secs(length_in_secs));
    println!("!!!!FINISHED!!!!")
}

