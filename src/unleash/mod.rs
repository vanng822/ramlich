use enum_map::Enum;
use log::info;
use once_cell::sync::OnceCell;
use reqwest;
use serde::{Deserialize, Serialize};
use unleash_api_client::{client, Client};

static INSTANCE: OnceCell<Client<UserFeatures, reqwest::Client>> = OnceCell::new();

pub fn getunleash() -> &'static Client<UserFeatures, reqwest::Client> {
    return INSTANCE.get().unwrap();
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Enum, Clone)]
pub enum UserFeatures {
    default,
}

pub async fn init_client() {
    let api_url = "http://127.0.0.1:4242/api";
    let app_name = "ramlich";
    let instance_id = "";
    let authorization = Some("default:development.unleash-insecure-api-token".to_string());
    let client_builder = client::ClientBuilder::default();
    let client = client_builder
        .into_client::<UserFeatures, reqwest::Client>(api_url, app_name, instance_id, authorization)
        .unwrap();
    let result = client.register().await;
    info!("{:?}", result);
    let _ = INSTANCE.set(client);
}