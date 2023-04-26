use std::env;
use std::fmt;

#[derive(Debug)]
pub struct Configuration {
    pub azure_tenant_id: String,
    pub azure_client_id: String,
    pub azure_client_secret: String,
    pub azure_subscription_id: String,
    pub azure_resource_group_name: String,
    pub azure_key_vault_name: String,
    pub azure_devops_org: String,
    pub azure_devops_proj: String,
    pub azure_devops_repository_id: String,
    pub azure_devops_email: String,
    pub azure_devops_personal_access_token: String,
    pub appsettings_json_path: String,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Azure Tenant ID: {}\n", self.azure_tenant_id)?;
        write!(f, "Azure Client ID: {}\n", self.azure_client_id)?;
        write!(f, "Azure Client Secret: {}\n", self.azure_client_secret)?;
        write!(f, "Azure Subscription ID: {}\n", self.azure_subscription_id)?;
        write!(
            f,
            "Azure Resource Group Name: {}\n",
            self.azure_resource_group_name
        )?;
        write!(f, "Azure Key Vault Name: {}", self.azure_key_vault_name)?;
        write!(f, "Azure DevOps Organization: {}", self.azure_devops_org)?;
        write!(f, "Azure DevOps Project: {}", self.azure_devops_proj)?;
        write!(
            f,
            "Azure DevOps Repository ID: {}",
            self.azure_devops_repository_id
        )?;
        write!(f, "Azure DevOps Email: {}", self.azure_devops_email)?;
        write!(
            f,
            "Azure DevOps Personal Access Token: {}",
            self.azure_devops_personal_access_token
        )?;
        write!(f, "AppSettings JSON Path: {}", self.appsettings_json_path)
    }
}

pub fn get_configuration() -> Configuration {
    let res = Configuration {
        azure_tenant_id: env::var("AZURE_TENANT_ID").unwrap(),
        azure_client_id: env::var("AZURE_CLIENT_ID").unwrap(),
        azure_client_secret: env::var("AZURE_CLIENT_SECRET").unwrap(),
        azure_subscription_id: env::var("AZURE_SUBSCRIPTION_ID").unwrap(),
        azure_resource_group_name: env::var("AZURE_RESOURCE_GROUP_NAME").unwrap(),
        azure_key_vault_name: env::var("AZURE_KEY_VAULT_NAME").unwrap(),
        azure_devops_org: env::var("AZURE_DEVOPS_ORGANIZATION").unwrap(),
        azure_devops_proj: env::var("AZURE_DEVOPS_PROJECT").unwrap(),
        azure_devops_repository_id: env::var("AZURE_DEVOPS_REPO_ID").unwrap(),
        azure_devops_email: env::var("AZURE_DEVOPS_EMAIL").unwrap(),
        azure_devops_personal_access_token: env::var("AZURE_DEVOPS_PERSONAL_ACCESS_TOKEN").unwrap(),
        appsettings_json_path: env::var("APPSETTINGS_JSON_PATH").unwrap(),
    };

    return res;
}
