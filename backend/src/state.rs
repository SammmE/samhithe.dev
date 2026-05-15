use std::sync::Arc;

use reqwest::Client;

use crate::{config::Config, firestore::Firestore};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub firestore: Firestore,
    pub http: Client,
}

impl AppState {
    pub fn new(config: Arc<Config>, firestore: Firestore) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("HTTP client configuration should be valid");

        Self {
            config,
            firestore,
            http,
        }
    }
}
