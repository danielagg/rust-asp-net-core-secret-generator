mod appsettings_parser;
mod configuration;
mod keyvault;

use appsettings_parser::{
    convert_appsettings_entries_to_keyvault_secret_names, get_appsettings_content,
};
use configuration::get_configuration;
use dotenv::dotenv;
use keyvault::fetch_secrets_from_azure_keyvault;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = get_configuration();

    let appsettings = get_appsettings_content(&config).await?;
    let secrets_to_fetch = convert_appsettings_entries_to_keyvault_secret_names(&appsettings);

    let secrets = fetch_secrets_from_azure_keyvault(secrets_to_fetch, &config).await?;

    for secret in secrets {
        println!("{}: {}", secret.key, secret.value);
    }

    Ok(())
}
