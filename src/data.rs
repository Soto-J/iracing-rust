use base64::{engine::general_purpose, Engine};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
const POKEMON_API: &str = "https://pokeapi.co/api/v2/pokemon/ditto";
const IRACING_AUTH_URL: &str = "https://members-ng.iracing.com/auth";
const IRACING_ENDPOINT: &str = "https://members-ng.iracing.com/data/doc";

#[allow(dead_code)]
#[derive(Deserialize)]
struct Data {
    data: String,
}

pub struct IracingClient {
    client: reqwest::Client,
}
impl IracingClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let email = std::env::var("IRACING_EMAIL")?;
        let password = Self::encoded_password();
        println!("Email = {} password = {}", email, password);

        let client = reqwest::Client::builder().cookie_store(true).build()?;

        //  Pass email and encoded-password to body for authentication
        let body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = client
            .post(IRACING_AUTH_URL)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?;

        let res_text = &response.text().await?;
        println!("[FROM NEW] = {:?}", res_text);

        Ok(IracingClient { client })
    }

    pub async fn get_iracing_data(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(IRACING_ENDPOINT).send().await?;

        let res_text = response.text().await?;

        let data: Value = serde_json::from_str(&res_text)?;

        println!("[FROM GET_IRACING_DATA] = {}", data);

        Ok(())
    }

    fn encoded_password() -> String {
        dotenv::dotenv().ok();

        let email = std::env::var("IRACING_EMAIL").expect("[Error]: Email Not Found!");
        let password = std::env::var("IRACING_PASSWORD").expect("[Error]: Password not Found!");

        let mut hasher = Sha256::new();
        hasher.update(format!("{password}{email}"));

        general_purpose::STANDARD.encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_password_test() {
        let password = "MyPassWord";
        let email = "clunky@iracing.com";

        let mut hasher = Sha256::new();
        hasher.update(format!("{password}{email}"));

        let result = general_purpose::STANDARD.encode(hasher.finalize());

        assert_eq!(result, "xGKecAR27ALXNuMLsGaG0v5Q9pSs2tZTZRKNgmHMg+Q=");
    }

    #[test]
    fn encoded_password_works() {
        let password = self::IracingClient::encoded_password();

        assert_eq!(password, "T/hmFXJGKQRxZZgT+n1kWzLcGXXW9yuqqs1oyUgfd/Q=");
    }
}

// *************** //
// Blocking Method //
// *************** //
// pub fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
//     let response = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon/ditto")?;

//     let body = response.text()?;

//     println!("Body: {:?}", body);

//     Ok(())
// }
