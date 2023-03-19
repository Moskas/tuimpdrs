use std::io::{stdin, stdout, Write};
pub fn user_prompt() -> String {
    let mut choice: String = String::new();
    stdin().read_line(&mut choice).unwrap();
    return choice.trim().to_string();
}

pub fn set_volume(connection: &mut mpd::Client) {
    print!("Set volume: ");
    stdout().flush().unwrap();
    let vol: i8 = user_prompt().parse::<i8>().unwrap();
    //println!("{}", vol as i32);
    match connection.volume(vol) {
        Ok(_ok) => println!("Volume set to: {}%", vol),
        Err(err) => println!("Failed to set the volume ({})", err),
    };
}

pub fn db_update(connection: &mut mpd::Client) {
    match connection.update() {
        Ok(_ok) => println!("Database updated"),
        Err(err) => println!("Database updtade failed {}", err),
    }
}

pub fn play_next(connection: &mut mpd::Client) {
    match connection.next() {
        Ok(_ok) => (),
        Err(err) => println!("Nothing to play next {}", err),
    };
}

pub fn play_previous(connection: &mut mpd::Client) {
    match connection.prev() {
        Ok(_ok) => (),
        Err(err) => println!("Nothing to rewind to {}", err),
    };
}

pub fn play_pause(connection: &mut mpd::Client) {
    match connection.toggle_pause() {
        Ok(_ok) => (),
        Err(err) => println!("Nothing playing ({})", err),
    };
}
