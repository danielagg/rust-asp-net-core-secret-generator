mod configuration;

use anyhow::anyhow;
use configuration::{get_configuration, Configuration};
use dotenv::dotenv;
use reqwest::{self, header};
use serde_json::{Map, Value};
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = get_configuration();

    let appsettings = get_appsettings_content(&config).await?;
    let secrets_to_fetch = convert_appsettings_entries_to_keyvault_secret_names(&appsettings);

    let azure_management_access_token = get_azure_management_access_token(&config).await?;

    // let url = format!(
    //     "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2022-07-01",
    //     config.subscription_id,
    //     config.resource_group_name,
    //     config.key_vault_name);

    // let client = reqwest::Client::new();
    // let response = client
    //     .get(url.as_str())
    //     .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
    //     .send()
    //     .await?;

    // let resp = response.text().await?;

    // println!("{}", resp);
    Ok(())
}

async fn get_appsettings_content(config: &Configuration) -> anyhow::Result<String> {
    let devops_url = format!(
        "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/items?path={}&api-version=6.0",
        config.azure_devops_org,
        config.azure_devops_proj,
        config.azure_devops_repository_id,
        config.appsettings_json_path
    );

    let client = reqwest::Client::new();
    let response = client
        .get(devops_url.as_str())
        .basic_auth(
            config.azure_devops_email.clone(),
            Some(config.azure_devops_personal_access_token.clone()),
        )
        .send()
        .await?;

    let resp = response.text().await?;

    Ok(resp)
}

async fn get_azure_management_access_token(config: &Configuration) -> anyhow::Result<String> {
    let body = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}&resource=https://management.azure.com",
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

    return Ok(access_token.to_owned());
}

fn convert_appsettings_entries_to_keyvault_secret_names(json_str: &str) -> Vec<String> {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");

    let mut result: Vec<String> = Vec::new();

    populate_name_of_secrets_to_be_injected(&json, None, &mut result);

    return result;
}

fn populate_name_of_secrets_to_be_injected(
    json: &Value,
    prefix: Option<&str>,
    result: &mut Vec<String>,
) {
    match json {
        Value::Null | Value::Bool(_) | Value::Number(_) => {
            if let Some(key) = prefix {
                result.push(key.to_string());
            }
        }
        Value::String(s) => {
            if let Some(key) = prefix {
                if s == "{InjectedFromKeyvaultDuringRelease}" {
                    result.push(key.to_string());
                }
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let key = prefix.map_or_else(|| i.to_string(), |p| format!("{}-{}", p, i));
                populate_name_of_secrets_to_be_injected(item, Some(&key), result);
            }
        }
        Value::Object(obj) => {
            for (key, value) in obj.iter() {
                let new_prefix =
                    prefix.map_or_else(|| key.to_string(), |p| format!("{}-{}", p, key));
                populate_name_of_secrets_to_be_injected(value, Some(&new_prefix), result);
            }
        }
    }
}
