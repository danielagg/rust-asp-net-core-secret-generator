use crate::configuration::Configuration;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

        println!("Secret URL: {}", secret_url);
        println!("Access Token: {}", access_token);

        let response = http_client
            .get(&secret_url)
            .bearer_auth(access_token)
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json = serde_json::from_str(&response_text)?;

        println!("Response JSON: {}", serde_json::to_string(&response_json)?);

        // let secret: AzureKeyVaultSecretValue = http_client
        //     .get(&secret_url)
        //     .bearer_auth(access_token)
        //     .send()
        //     .await?
        //     .json()
        //     .await?;

        // secret_values.push(Secret {
        //     key: secret_name,
        //     value: secret.value,
        // });
    }

    Ok(secret_values)
}
