extern crate mpd;
mod prompt;

use mpd::Client;
use prompt::user_prompt;
fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    conn.volume(100).unwrap();
    //println!("Status: {:?}", conn.status());
    // Now playing function
    println!(
        "Now Playing: {} - {} from {}",
        conn.currentsong()
            .unwrap()
            .unwrap()
            .tags
            .get("Artist")
            .unwrap(),
        conn.currentsong().unwrap().unwrap().title.unwrap(),
        conn.currentsong()
            .unwrap()
            .unwrap()
            .tags
            .get("Album")
            .unwrap()
    );
    conn.update().unwrap(); // it works woo
    print!("{}", user_prompt());
}
