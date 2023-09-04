use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher};

use reqwest::{self, Response};

use scraper::{Html, Selector};

use std::fmt;
use std::path::PathBuf;

pub fn listen_for_slp_creation(slp_path: PathBuf, player_code: ConnectCode) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(&slp_path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => get_opponent_connect_code(),
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}

pub fn get_opponent_connect_code() {}

pub fn get_connect_code_page_data(code: ConnectCode) -> reqwest::Result<String> {
    reqwest::blocking::get(format!(
        "https://slippi.gg/user/{}-{}",
        code.name, code.discriminant
    ))?
    .text()
}

pub fn get_win_loss_data(page_data: String) -> WinLossData {
    let document = scraper::Html::parse_document(&page_data);

    let wins_selector =
        scraper::Selector::parse("MuiTypography-root MuiTypography-body1 css-1i7pcxu").unwrap();

    let wins = document.select(&wins_selector).map(|x| x.inner_html());

    //println!("{:?}", wins);

    WinLossData::new(0, 0)
}

pub struct ConnectCode {
    name: String,
    discriminant: usize,
}

pub struct WinLossData {
    wins: usize,
    losses: usize,
}

impl ConnectCode {
    pub fn new(name: String, discriminant: usize) -> Self {
        Self {
            name: name,
            discriminant: discriminant,
        }
    }
}

impl WinLossData {
    pub fn new(wins: usize, losses: usize) -> Self {
        Self { wins, losses }
    }
}

impl fmt::Display for ConnectCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.name, self.discriminant)
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_connect_code_page_data, get_win_loss_data, ConnectCode};
    use std::fmt;
    #[test]
    fn get_slippi_page_test() {
        let cody = ConnectCode::new("IBDW".to_string(), 0);
        println!("{:?}", get_connect_code_page_data(cody))
    }
    #[test]
    fn connect_code_display_test() {
        let cody = ConnectCode::new("IBDW".to_string(), 0);
        assert_eq!(format!("{}", cody), "IBDW#0");
    }
    #[test]
    fn get_win_loss_data_test() {
        let cody = ConnectCode::new("IBDW".to_string(), 0);
        let page_data = get_connect_code_page_data(cody).unwrap();
        println!("Page Data: {}", page_data);
        get_win_loss_data(page_data);
    }
}
