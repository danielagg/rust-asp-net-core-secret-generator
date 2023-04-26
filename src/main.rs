mod configuration;

use anyhow::anyhow;
use configuration::{get_configuration, Configuration};
use dotenv::dotenv;
use reqwest::{self, header};
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = get_configuration();
    let access_token = get_access_token(&config).await?;

    let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2022-07-01",
        config.subscription_id,
        config.resource_group_name,
        config.key_vault_name);

    let client = reqwest::Client::new();
    let response = client
        .get(url.as_str())
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?;

    let resp = response.text().await?;

    println!("{}", resp);
    Ok(())
}

async fn get_access_token(config: &Configuration) -> anyhow::Result<String> {
    let body = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}&resource=https://management.azure.com",
        config.client_id, config.client_secret
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&format!(
            "https://login.microsoftonline.com/{}/oauth2/token",
            config.tenant_id
        ))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    let response_body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&response_body)?;

    let access_token = json["access_token"]
        .as_str()
        .ok_or_else(|| anyhow!("access_token not found"))?;

    return Ok(access_token.to_owned());
}
