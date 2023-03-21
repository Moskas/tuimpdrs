mod config;
mod now_playing;
mod prompt;
mod songlist;
use std::{
    io::{stdout, Write},
    process::exit,
};

use config::get_config;
use mpd::Client;
use now_playing::{get_artist, get_title};
use prompt::{db_update, play_next, play_previous, set_volume, user_prompt};
use songlist::{get_queue, get_song_titles_from_queue};

use crate::{now_playing::get_albumname, prompt::play_pause};

fn main() {
    let config = get_config();
    loop {
        match Client::connect(&format!("{}:{}", config.ip, config.port)[..]) {
            Ok(connected) => {
                let mut conn = connected;
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
                                //.to_string()
                                //.split('.')
                                //.into_iter()
                                //.map(|s| s.parse::<f64>().unwrap() / 60.)
                                //.collect::<Vec<f64>>(),
                                //.into_iter()
                                //.sum::<f64>(),
                                now_playing::get_year(&mut conn)
                            );
                        }
                        "v" => set_volume(&mut conn),
                        "u" => db_update(&mut conn),
                        "n" => play_next(&mut conn),
                        "p" => play_previous(&mut conn),
                        "t" => play_pause(&mut conn),
                        "h" => {
                            println!(
                                "Available commands:
v - set volume
u - update mpd database
n - play next song from queue
p - play previous song from queue
t - toggle between playing and pause
l - list current queue
q - quit from the prompt"
                            )
                        }
                        "l" => get_song_titles_from_queue(get_queue(&mut conn))
                            .into_iter()
                            .map(|x| print!("{}", x))
                            .collect(),
                        "q" => exit(0),
                        _ => println!("Not a function"),
                    }
                }
            }
            Err(error) => {
                println!("Couldn't connect, is mpd running? {}", error);
                std::thread::sleep(std::time::Duration::from_millis(5000)); // 5 second timeout before doing another connection check
            }
        }
    }
}
