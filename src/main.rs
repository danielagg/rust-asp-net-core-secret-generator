use anyhow::anyhow;
use dotenv::dotenv;
use reqwest::{self, header};
use std::env;
use std::fmt;
use tokio; // Import `fmt`

#[derive(Debug)]
struct Configuration {
    tenant_id: String,
    client_id: String,
    client_secret: String,
    subscription_id: String,
    resource_group_name: String,
    key_vault_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = get_configuration();

    println!("{}", config);

    // let access_token = get_access_token().await?;

    // let url = format!(
    //     "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}?api-version=2022-07-01",
    //     subscription_id,
    //     resource_group_name,
    //     key_vault_name);

    // let formatted_url = url.as_str();

    // println!("{}", formatted_url);

    // let resp = reqwest::get(formatted_url).await?.text().await?;
    // println!("{}", resp);
    Ok(())
}

fn get_configuration() -> Configuration {
    let res = Configuration {
        tenant_id: env::var("AZURE_CLIENT_ID").unwrap(),
        client_id: String::from("1"),
        client_secret: String::from("1"),
        subscription_id: String::from("1"),
        resource_group_name: String::from("1"),
        key_vault_name: String::from("1"),
    };

    return res;
}

// async fn get_access_token() -> anyhow::Result<String> {
//     let body = format!(
//         "grant_type=client_credentials&client_id={}&client_secret={}&resource={}",
//         client_id, client_secret, resource
//     );

//     let client = reqwest::Client::new();
//     let response = client
//         .post(&format!(
//             "https://login.microsoftonline.com/{}/oauth2/token",
//             tenant_id
//         ))
//         .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await?;

//     let response_body = response.text().await?;
//     let json: serde_json::Value = serde_json::from_str(&response_body)?;
//     let access_token = json["access_token"]
//         .as_str()
//         .ok_or_else(|| anyhow!("access_token not found"))?;

//     println!("{}", access_token);

//     return Ok(access_token.to_owned());
// }
