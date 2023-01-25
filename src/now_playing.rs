use mpd::Client;
pub fn get_title(connection: &mut Client) -> String {
    connection.currentsong().unwrap().unwrap().title.unwrap()
}

pub fn get_artist(connection: &mut Client) -> String {
    connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("Artist")
        .unwrap()
        .to_string()
}

pub fn get_albumname(connection: &mut Client) -> String {
    connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("Album")
        .unwrap()
        .to_string()
}
