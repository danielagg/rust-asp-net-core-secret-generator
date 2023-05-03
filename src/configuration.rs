use std::env;

#[derive(Debug)]
pub struct Configuration {
    pub azure_tenant_id: String,
    pub azure_client_id: String,
    pub azure_client_secret: String,
    pub azure_subscription_id: String,
    pub azure_resource_group_name: String,
    pub azure_key_vault_base_url: String,
    pub azure_devops_org: String,
    pub azure_devops_proj: String,
    pub azure_devops_repository_id: String,
    pub azure_devops_email: String,
    pub azure_devops_personal_access_token: String,
    pub appsettings_json_path: String,
}

pub fn get_configuration() -> Configuration {
    let res = Configuration {
        azure_tenant_id: env::var("AZURE_TENANT_ID").unwrap(),
        azure_client_id: env::var("AZURE_CLIENT_ID").unwrap(),
        azure_client_secret: env::var("AZURE_CLIENT_SECRET").unwrap(),
        azure_subscription_id: env::var("AZURE_SUBSCRIPTION_ID").unwrap(),
        azure_resource_group_name: env::var("AZURE_RESOURCE_GROUP_NAME").unwrap(),
        azure_key_vault_base_url: env::var("AZURE_KEY_VAULT_BASE_URL").unwrap(),
        azure_devops_org: env::var("AZURE_DEVOPS_ORGANIZATION").unwrap(),
        azure_devops_proj: env::var("AZURE_DEVOPS_PROJECT").unwrap(),
        azure_devops_repository_id: env::var("AZURE_DEVOPS_REPO_ID").unwrap(),
        azure_devops_email: env::var("AZURE_DEVOPS_EMAIL").unwrap(),
        azure_devops_personal_access_token: env::var("AZURE_DEVOPS_PERSONAL_ACCESS_TOKEN").unwrap(),
        appsettings_json_path: env::var("APPSETTINGS_JSON_PATH").unwrap(),
    };

    return res;
}
