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
    println!("{}", vol as i32);
    connection.volume(vol).unwrap();
    // For some reason it's not rounding up properly
    // for example vol = 50 in mpd is vol 49%
    println!("Volume set to: {}%", vol);
}

pub fn db_update(connection: &mut mpd::Client) {
    connection.update().unwrap();
    println!("Database updated");
}

pub fn play_next(connection: &mut mpd::Client) {
    connection.next().unwrap();
}

pub fn play_previous(connection: &mut mpd::Client) {
    connection.prev().unwrap();
}

pub fn play_pause(connection: &mut mpd::Client) {
    connection.toggle_pause().unwrap();
}
