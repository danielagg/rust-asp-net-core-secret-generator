use crate::configuration::Configuration;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::Client;
use serde_json::Deserialize;

#[derive(Debug, Deserialize)]
struct Secret {
    value: String,
}

pub async fn fetch_secrets_from_azure_keyvault(
    secrets: Vec<String>,
    access_token: &str,
) -> Result<()> {
    let http_client = Client::new();

    // todo:
    let vault_url = std::env::var("AZURE_KEYVAULT_URL")?;

    for secret_name in secrets {
        let secret_url = format!("{}/secrets/{}?api-version=7.2", vault_url, secret_name);
        let secret: Secret = http_client
            .get(&secret_url)
            .bearer_auth(access_token.as_str())
            .send()
            .await?
            .json()
            .await?;

        println!("{}: {}", secret_name, secret.value);
    }

    Ok(())
}
