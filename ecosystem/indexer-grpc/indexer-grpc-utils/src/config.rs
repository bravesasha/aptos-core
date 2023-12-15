// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::storage_format::StorageFormat;
use serde::{Deserialize, Serialize};
/// Common configuration for Indexer GRPC Store.
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GcsFileStore {
    pub gcs_file_store_bucket_name: String,
    // Required to operate on GCS.
    pub gcs_file_store_service_account_key_path: String,

    storage_format: StorageFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalFileStore {
    pub local_file_store_path: PathBuf,
    storage_format: StorageFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "file_store_type")]
pub enum IndexerGrpcFileStoreConfig {
    GcsFileStore(GcsFileStore),
    LocalFileStore(LocalFileStore),
}

impl IndexerGrpcFileStoreConfig {
    pub fn create(&self) -> Box<dyn crate::file_store_operator::FileStoreOperator> {
        match self {
            IndexerGrpcFileStoreConfig::GcsFileStore(gcs_file_store) => {
                Box::new(crate::file_store_operator::gcs::GcsFileStoreOperator::new(
                    gcs_file_store.gcs_file_store_bucket_name.clone(),
                    gcs_file_store
                        .gcs_file_store_service_account_key_path
                        .clone(),
                    gcs_file_store.storage_format,
                ))
            },
            IndexerGrpcFileStoreConfig::LocalFileStore(local_file_store) => Box::new(
                crate::file_store_operator::local::LocalFileStoreOperator::new(
                    local_file_store.local_file_store_path.clone(),
                    local_file_store.storage_format,
                ),
            ),
        }
    }
}

impl Default for IndexerGrpcFileStoreConfig {
    fn default() -> Self {
        IndexerGrpcFileStoreConfig::LocalFileStore(LocalFileStore {
            local_file_store_path: std::env::current_dir().unwrap(),
            storage_format: StorageFormat::JsonBase64UncompressedProto,
        })
    }
}
