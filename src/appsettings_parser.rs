use crate::configuration::Configuration;
use serde_json::Value;

pub async fn get_appsettings_content(config: &Configuration) -> anyhow::Result<String> {
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

pub fn convert_appsettings_entries_to_keyvault_secret_names(json_str: &str) -> Vec<String> {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");

    let mut result: Vec<String> = Vec::new();

    populate_name_of_secrets_to_be_injected(&json, None, &mut result);

    return result;
}

pub fn populate_name_of_secrets_to_be_injected(
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
