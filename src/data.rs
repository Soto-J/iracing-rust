use base64::{engine::general_purpose, Engine};
use dotenv::dotenv;
use reqwest::{self, blocking::Response, get, Error};
use sha2::{Digest, Sha256};

pub fn fetch_ircaing_data() -> Result<Response, Error> {
    let url = get_url();

    reqwest::blocking::get(url)
}

fn get_url() -> String {
    dotenv().ok();

    let email = std::env::var("IRACING_EMAIL").expect("Not Found!");
    let password = std::env::var("IRACING_PASSWORD").expect("Not Found!");

    let mut hasher = Sha256::new();

    hasher.update(format!("{password}{email}"));

    format!(
        "https://members-ng.iracing.com/{}",
        general_purpose::STANDARD.encode(hasher.finalize())
    )
}
