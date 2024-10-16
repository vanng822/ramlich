use std::{thread, time::Duration};

use enum_map::Enum;
use futures_timer::Delay;
use log::info;
use once_cell::sync::OnceCell;
use reqwest;
use serde::{Deserialize, Serialize};
use unleash_api_client::{client, Client};

static INSTANCE: OnceCell<Client<UserFeatures, reqwest::Client>> = OnceCell::new();

pub fn getunleash() -> &'static Client<UserFeatures, reqwest::Client> {
    INSTANCE
        .get()
        .expect("Should have a setup of unleash client")
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Enum, Clone)]
pub enum UserFeatures {
    default,
    request_event,
    request_event_enabled,
}

pub async fn init_client(app_name: &str, api_url: &str, authorization: Option<String>) {
    let instance_id = "";
    let client_builder = client::ClientBuilder::default();
    let client = client_builder
        .into_client::<UserFeatures, reqwest::Client>(api_url, app_name, instance_id, authorization)
        .expect("Created unleash client");
    let result = client.register().await;
    info!("{:?}", result);

    if result.is_err() {
        panic!("Can not register client");
    }

    let _ = INSTANCE.set(client);
}

pub async fn sync_features() {
    let client = getunleash();
    info!("sync_features thread id: {:?}", thread::current().id());
    futures::future::join(client.poll_for_updates(), async {
        Delay::new(Duration::from_millis(500)).await;
        println!(
            "feature 'default' is {}",
            client.is_enabled(UserFeatures::default, None, false)
        );
        println!(
            "feature 'request_event_enabled' is {}",
            client.is_enabled(UserFeatures::request_event_enabled, None, false)
        );
        // Wait to allow metrics upload
        Delay::new(Duration::from_millis(500)).await;
        // allow the future::join to finish
    })
    .await;
}
