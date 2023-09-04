use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use reqwest::{self, Response};

use scraper::{Html, Selector};

use peppi::serde::de::Opts;

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

pub fn listen_for_slp_creation(
    slp_directory: PathBuf,
    player_code: ConnectCode,
) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(&slp_directory, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                let opp_code = get_opponent_connect_code(&event.paths[0], &player_code);
                println!("opp_code: {}", opp_code);
            }
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}

pub fn get_opponent_connect_code(p: &PathBuf, player_code: &ConnectCode) -> ConnectCode {
    let mut buf = io::BufReader::new(fs::File::open(p).unwrap());
    let no_frames_options = Opts {
        skip_frames: true,
        debug_dir: None,
    };

    let game = peppi::game(&mut buf, Some(&no_frames_options), None).unwrap();
    let players = game.metadata.players.unwrap();

    let opp_code_string = if player_code.to_string() != players[0].netplay.as_ref().unwrap().code {
        &players[0].netplay.as_ref().unwrap().code
    } else {
        &players[1].netplay.as_ref().unwrap().code
    };

    ConnectCode::from_str(&opp_code_string).unwrap()
}

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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct ParseConnectCodeError;

impl FromStr for ConnectCode {
    type Err = ParseConnectCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str = s.split_once('#').unwrap();
        let (name, discriminant_str) = split_str;
        let discriminant = discriminant_str
            .parse::<usize>()
            .map_err(|_| ParseConnectCodeError)?;

        Ok(ConnectCode {
            name: name.to_string(),
            discriminant,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn connect_code_from_str_test() {
        let cody_str = "IBDW#0";
        let cody_code = ConnectCode::new("IBDW".to_string(), 0);
        let cody_from_str = ConnectCode::from_str(cody_str).unwrap();
        assert_eq!(cody_from_str, cody_code);
    }
}
