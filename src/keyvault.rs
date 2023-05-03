use crate::configuration::Configuration;
use anyhow::anyhow;
use reqwest::{header, Client};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct AzureKeyVaultSecretValue {
    value: String,
}

pub struct Secret {
    pub key: String,
    pub value: String,
}

pub async fn fetch_secrets_from_azure_keyvault(
    secrets: Vec<String>,
    config: &Configuration,
) -> anyhow::Result<Vec<Secret>> {
    let access_token = get_access_token(config).await?;

    let http_client = Client::new();
    let mut secret_values: Vec<Secret> = Vec::new();

    for secret_name in secrets {
        let secret_url = format!(
            "{}/secrets/{}?api-version=7.2",
            config.azure_key_vault_base_url, secret_name
        );

        let http_response = http_client
            .get(&secret_url)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|error| eprintln!("Could not fetch secret from KeyVault due to: {}", error));

        let secret_result = match http_response {
            Ok(response) => response.json::<AzureKeyVaultSecretValue>().await,
            Err(_) => continue,
        };

        match secret_result {
            Ok(secret) => {
                secret_values.push(Secret {
                    key: secret_name,
                    value: secret.value,
                });
            }
            Err(_) => continue,
        }
    }

    Ok(secret_values)
}

async fn get_access_token(config: &Configuration) -> anyhow::Result<String> {
    let body = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}&resource=https://vault.azure.net",
        config.azure_client_id, config.azure_client_secret
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&format!(
            "https://login.microsoftonline.com/{}/oauth2/token",
            config.azure_tenant_id
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

    Ok(access_token.to_owned())
}
