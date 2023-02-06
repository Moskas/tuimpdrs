extern crate mpd;
mod now_playing;
mod prompt;
use std::{
    io::{stdout, Write},
    process::exit,
};

use mpd::Client;
use now_playing::{get_artist, get_title};
use prompt::{db_update, play_next, play_previous, set_volume, user_prompt};

use crate::{now_playing::get_albumname, prompt::play_pause};

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    //println!("Status: {:?}", conn.status());
    // Now playing function
    loop {
        print!("MPD Prompt: ");
        stdout().flush().unwrap();
        match &user_prompt()[..] {
            "np" => {
                println!(
                    "Now Playing: {} - {} from album {}",
                    get_artist(&mut conn),
                    get_title(&mut conn),
                    get_albumname(&mut conn)
                );
            }
            "i" => {
                println!(
                    "{}s from year {}",
                    now_playing::get_length(&mut conn),
                    now_playing::get_year(&mut conn)
                );
            }
            "v" => set_volume(&mut conn),
            "u" => db_update(&mut conn),
            "n" => play_next(&mut conn),
            "p" => play_previous(&mut conn),
            "s" => play_pause(&mut conn),
            "h" => {
                println!("Available commands: \nv - set volume\nu - update mpd database\nn - play next song from queue\np - play previous song from queue\ns - toggle between playing and pause\nq - quit from the prompt")
            }
            "q" => exit(0),
            _ => println!("Not a function"),
        }
    }
}
