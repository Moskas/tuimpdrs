extern crate mpd;
mod now_playing;
mod prompt;
use mpd::Client;
use now_playing::{get_artist, get_title};
use prompt::{db_update, set_volume, user_prompt};

use crate::now_playing::get_albumname;

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    //conn.volume(100).unwrap();
    //println!("Status: {:?}", conn.status());
    // Now playing function
    match &user_prompt()[..] {
        "np" => {
            println!(
                "Now Playing: {} - {} from {}",
                get_artist(&mut conn),
                get_title(&mut conn),
                get_albumname(&mut conn)
            );
        }
        "v" => set_volume(&mut conn),
        "u" => db_update(&mut conn),
        _ => println!("Not a function"),
    }
    //conn.update().unwrap(); // it works woo
}
