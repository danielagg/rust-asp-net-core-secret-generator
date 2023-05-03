use crate::configuration::Configuration;
use reqwest::Client;
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
    access_token: &str,
) -> anyhow::Result<Vec<Secret>> {
    let http_client = Client::new();
    let mut secret_values: Vec<Secret> = Vec::new();

    for secret_name in secrets {
        let secret_url = format!(
            "{}/secrets/{}?api-version=7.2",
            config.azure_key_vault_base_url, secret_name
        );

        let http_response = http_client
            .get(&secret_url)
            .bearer_auth(access_token)
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
