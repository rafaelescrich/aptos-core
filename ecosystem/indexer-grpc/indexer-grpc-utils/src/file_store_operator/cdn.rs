// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{constants::BLOB_STORAGE_SIZE, file_store_operator::*, EncodedTransactionWithVersion};

pub struct CdnFileStoreOperator {
    url: String,
    pub client: reqwest::Client,
}

impl CdnFileStoreOperator {
    pub fn new(url: String) -> Self {
        let client = reqwest::Client::new();
        Self { url, client }
    }
}

#[async_trait::async_trait]
impl FileStoreOperator for CdnFileStoreOperator {
    /// Bootstraps the file store operator. This is required before any other operations.
    async fn verify_storage_bucket_existence(&self) {

    }

    /// Gets the transactions files from the file store. version has to be a multiple of BLOB_STORAGE_SIZE.
    async fn get_transactions(&self, version: u64) -> anyhow::Result<Vec<String>> {
        let batch_start_version = version / BLOB_STORAGE_SIZE as u64 * BLOB_STORAGE_SIZE as u64;
        let current_file_name = generate_blob_name(batch_start_version);
        // Use reqwest to download the file.
        let url = format!("{}/{}", self.url, current_file_name);
        tracing::info!("Downloading transactions file from {}", url);
        self.client
            .get(url.as_str())
            .send()
            .await?
            .bytes()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to download transactions file: {}", e))
            .map(|bytes| {
                let transactions_file: TransactionsFile = serde_json::from_slice(&bytes)
                    .expect("Expected transactions file to be valid JSON.");
                transactions_file.transactions
            })
    }

    /// Gets the raw transactions file from the file store. Mainly for verification purpose.
    async fn get_raw_transactions(&self, version: u64) -> anyhow::Result<TransactionsFile> {
        let batch_start_version = version / BLOB_STORAGE_SIZE as u64 * BLOB_STORAGE_SIZE as u64;
        let current_file_name = generate_blob_name(batch_start_version);
        // Use reqwest to download the file.
        self.client
            .get(format!("{}/files/{}", self.url, current_file_name).as_str())
            .send()
            .await?
            .bytes()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to download transactions file: {}", e))
            .map(|bytes| {
                serde_json::from_slice(&bytes)
                    .expect("Expected transactions file to be valid JSON.")
            })
    }

    /// Gets the metadata from the file store. Operator will panic if error happens when accessing the metadata file(except not found).
    async fn get_file_store_metadata(&self) -> Option<FileStoreMetadata> {
        unimplemented!();
    }

    /// If the file store is empty, the metadata will be created; otherwise, return the existing metadata.
    async fn create_default_file_store_metadata_if_absent(
        &mut self,
        _expected_chain_id: u64,
    ) -> anyhow::Result<FileStoreMetadata> {
        unimplemented!();
    }

    /// Updates the file store metadata. This is only performed by the operator when new file transactions are uploaded.
    async fn update_file_store_metadata(
        &mut self,
        _chain_id: u64,
        _version: u64,
    ) -> anyhow::Result<()> {
        unimplemented!();
    }

    /// Updates the verification metadata file.
    async fn update_verification_metadata(
        &mut self,
        _chain_id: u64,
        _next_version_to_verify: u64,
    ) -> Result<()> {
        unimplemented!();
    }

    /// Uploads the transactions to the file store. The transactions are grouped into batches of BLOB_STORAGE_SIZE.
    /// Updates the file store metadata after the upload.
    async fn upload_transactions(
        &mut self,
        _chain_id: u64,
        _transactions: Vec<EncodedTransactionWithVersion>,
    ) -> anyhow::Result<()> {
        unimplemented!();
    }

    async fn get_or_create_verification_metadata(
        &self,
        _chain_id: u64,
    ) -> Result<VerificationMetadata> {
        unimplemented!();
    }
}
