use std::env;
use std::fmt;

#[derive(Debug)]
pub struct Configuration {
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub subscription_id: String,
    pub resource_group_name: String,
    pub key_vault_name: String,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration:\n")?;
        write!(f, "  Tenant ID: {}\n", self.tenant_id)?;
        write!(f, "  Client ID: {}\n", self.client_id)?;
        write!(f, "  Client Secret: {}\n", self.client_secret)?;
        write!(f, "  Subscription ID: {}\n", self.subscription_id)?;
        write!(f, "  Resource Group Name: {}\n", self.resource_group_name)?;
        write!(f, "  Key Vault Name: {}", self.key_vault_name)
    }
}

pub fn get_configuration() -> Configuration {
    let res = Configuration {
        tenant_id: env::var("AZURE_TENANT_ID").unwrap(),
        client_id: env::var("AZURE_CLIENT_ID").unwrap(),
        client_secret: env::var("AZURE_CLIENT_SECRET").unwrap(),
        subscription_id: env::var("SUBSCRIPTION_ID").unwrap(),
        resource_group_name: env::var("RESOURCE_GROUP_NAME").unwrap(),
        key_vault_name: env::var("KEY_VAULT_NAME").unwrap(),
    };

    return res;
}
