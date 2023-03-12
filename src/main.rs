use inquire::Select;
use reqwest::blocking::{self, Response};
use rss::Channel;
use std::error::Error;

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

/* fn get_command() -> Command {
    let mut args = args();
    let program = args.nth(1).expect("missing program");

    let mut command = Command::new(program);
    command.args(args);
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command
} */

fn main() -> Result<(), Box<dyn Error>> {
    let animes = get_animes()?;
    let items: Vec<_> = animes
        .iter()
        .map(|anime| &anime.title)
        .map(|title| {
            title
                .trim_start_matches("[PuyaSubs!] ")
                .trim_end_matches(".mkv")
                .replace("[ESP-ENG]", "")
        })
        .collect();

    let selection = Select::new("Choose the anime", items).raw_prompt()?;

    /* let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    .items(&items)
    .default(0)
    .interact()?; */

    let selected_anime = &animes[selection.index];

    println!("{}", selected_anime.link);

    /* let mut child = get_command().spawn().expect("error executing command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let a = animes.clone();
    std::thread::spawn(move || {
        let x = a
            .iter()
            .map(|a| a.title.clone())
            .collect::<Vec<String>>()
            .join("\n");
        stdin
            .write_all(x.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    let anime = animes.iter().find(|a| a.title == output).expect("anime not found");

    println!("{}", anime.link); */

    Ok(())
}
