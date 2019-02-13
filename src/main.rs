use std::thread;
use std::time;

fn play_music() {
    std::iter::repeat(0).for_each(|_| {
        println!("Playing music!");
        thread::sleep(time::Duration::from_millis(500));
    });
}

fn main() {
    thread::spawn(play_music);
    thread::sleep(time::Duration::from_secs(5));
    println!("Playing ALARM!!!!")
}
