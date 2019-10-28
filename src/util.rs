use std::fs::File;
use clap::{App, Arg, ArgMatches};

/// Prints out general error message and exits the program "gracefully"
fn exit_gracefully<E: std::fmt::Debug, T: std::fmt::Debug>(msg: E) -> T {
    panic!("{:?}", msg);
}

/// Open file location and return File structure
pub fn open_file(file_loc: &str) -> File {
    return File::open(&file_loc).unwrap_or_else(|msg| {
        crate::util::exit_gracefully(format!(
            "Problem opening file: {} -- Error: {}",
            file_loc, msg
        ))
    });
}

/// Parse program arguments
/// The 'static means the ArgMatches lifetime will last the whole program
pub fn parse_args() -> ArgMatches<'static>{
    App::new("pomodoro_clock_rust")
        .version("0.1.0")
        .author("Paul Naranja")
        .about("A CLI pomodoro clock")
        .arg(
            Arg::with_name("length_in_minutes")
                .help("Length of pomodoro clock in minutes")
                .short("l")
                .default_value("1")
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
        .get_matches()
}
