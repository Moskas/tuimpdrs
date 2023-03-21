use mpd::{Client, Song};

pub fn get_queue(client: &mut Client) -> Vec<Song> {
    match client.queue() {
        Ok(queue) => queue,
        Err(_err) => Vec::<Song>::new(),
    }
}

pub fn get_song_titles_from_queue(queue: Vec<Song>) -> Vec<String> {
    queue
        .into_iter()
        .map(|x| format!("{} - {}\n", x.tags.get("Artist").unwrap(), x.title.unwrap()))
        .collect()
}
