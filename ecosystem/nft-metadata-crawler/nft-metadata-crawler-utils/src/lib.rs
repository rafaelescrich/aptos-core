// Copyright © Aptos Foundation

use chrono::{NaiveDateTime, Utc};
use google_cloud_auth::{
    project::{create_token_source, Config},
    token_source::TokenSource,
};

pub mod gcs;
pub mod pubsub;

/**
 * Struct to help with parsing of CSV
 */
#[derive(Clone, Debug)]
pub struct NFTMetadataCrawlerEntry {
    pub token_data_id: String,
    pub token_uri: String,
    pub last_transaction_version: i32,
    pub last_transaction_timestamp: chrono::NaiveDateTime,
    pub last_updated: chrono::NaiveDateTime,
    pub force: bool,
}

impl NFTMetadataCrawlerEntry {
    pub fn new(s: String) -> anyhow::Result<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        Ok(Self {
            token_data_id: parts[0].to_string(),
            token_uri: parts[1].to_string(),
            last_transaction_version: parts[2].to_string().parse()?,
            last_transaction_timestamp: NaiveDateTime::parse_from_str(
                parts[3],
                "%Y-%m-%d %H:%M:%S %Z",
            )
            .unwrap_or(NaiveDateTime::parse_from_str(
                parts[3],
                "%Y-%m-%d %H:%M:%S%.f %Z",
            )?),
            last_updated: Utc::now().naive_utc(),
            force: parts[4].parse::<bool>().unwrap_or(false),
        })
    }
}

pub async fn get_token_source() -> Box<dyn TokenSource> {
    create_token_source(Config {
        audience: None,
        scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
        sub: None,
    })
    .await
    .expect("No token source")
}