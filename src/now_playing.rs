use mpd::Client;

pub fn get_title(connection: &mut Client) -> String {
    match connection.currentsong().unwrap().unwrap().title {
        Some(title) => title,
        None => format!("None"),
    }
}

pub fn get_artist(connection: &mut Client) -> String {
    match connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("Artist")
    {
        Some(artist) => artist.to_string(),
        None => "None".to_string(),
    }
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

pub fn get_year(connection: &mut Client) -> i32 {
    connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("Date")
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

pub fn get_length(connection: &mut Client) -> f64 {
    connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("duration")
        .unwrap()
        .parse::<f64>()
        .unwrap()
}
