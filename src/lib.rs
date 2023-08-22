use reqwest::{self, Response};
use scraper::{Html, Selector};
use std::fmt;

pub fn get_connect_code_page_data(code: ConnectCode) -> reqwest::Result<String> {
    reqwest::blocking::get(format!(
        "https://slippi.gg/user/{}-{}",
        code.name, code.discriminant
    ))?
    .text()
}

pub struct ConnectCode {
    name: String,
    discriminant: usize,
}

impl ConnectCode {
    pub fn new(name: String, discriminant: usize) -> Self {
        Self {
            name: name,
            discriminant: discriminant,
        }
    }
}

impl fmt::Display for ConnectCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.name, self.discriminant)
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_connect_code_page_data, ConnectCode};
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
}
