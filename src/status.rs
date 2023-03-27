use mpd::{Client, Status};

pub fn get_mpd_status(connection: &mut Client) -> Status {
    connection.status().unwrap()
}

pub fn get_length(mpd_status: Status) -> String {
    //let (current, total) = mpd_status.time.unwrap();
    let length: String = format!(
        "{}:{}",
        mpd_status.duration.unwrap().num_minutes(),
        mpd_status.duration.unwrap().num_seconds() % 60 // it returns only WHOLE seconds, so if song has something like 59.42 second the decimal will be ignored
    );
    //println!("{length}");
    length
}
