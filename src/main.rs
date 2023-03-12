use inquire::Select;
use reqwest::blocking::{self, Response};
use rss::Channel;
use std::{error::Error, fmt};

fn fetch_data() -> Response {
    blocking::get("https://nyaa.si/?page=rss&u=puyero").expect("Error fetching data")
}

#[derive(Debug, Clone)]
pub struct Anime {
    title: String,
    link: String,
}

fn get_animes() -> Result<Vec<Anime>, Box<dyn Error>> {
    let res = fetch_data();
    let channel = Channel::read_from(&res.bytes()?[..])?;

    let mut animes = Vec::new();

    for item in channel.items() {
        let title = item.title.as_ref().expect("item missing title").to_string();
        let link = item.link.as_ref().expect("item missing link").to_string();
        let anime = Anime { title, link };
        animes.push(anime);
    }

    Ok(animes)
}

fn main() -> Result<(), Box<dyn Error>> {
    let animes = get_animes()?;
    let selected_anime = Select::new("Choose the anime", animes).prompt()?;
    println!("{}", selected_anime.link);

    Ok(())
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self
            .title
            .trim_start_matches("[PuyaSubs!] ")
            .trim_end_matches(".mkv")
            .replace("[ESP-ENG]", "");
        write!(f, "{}", title)
    }
}
