// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::loader::Script;
use move_binary_format::{errors::PartialVMResult, file_format::CompiledScript};
use sha3::{Digest, Sha3_256};
use std::sync::Arc;

// TODO(George) remove pub here for inner type.
pub struct ScriptHash(pub [u8; 32]);

impl From<&[u8]> for ScriptHash {
    fn from(serialized_script: &[u8]) -> Self {
        let mut sha3_256 = Sha3_256::new();
        sha3_256.update(serialized_script);
        let hash_value: [u8; 32] = sha3_256.finalize().into();
        Self(hash_value)
    }
}

pub trait ScriptStorage {
    fn fetch_deserialized_script(
        &self,
        serialized_script: &[u8],
    ) -> PartialVMResult<Arc<CompiledScript>>;

    fn fetch_or_create_verified_script(
        &self,
        serialized_script: &[u8],
        f: &dyn Fn(Arc<CompiledScript>) -> PartialVMResult<Script>,
    ) -> PartialVMResult<Arc<Script>>;

    // Panics if the script has not been created and cached before.
    fn fetch_existing_verified_script(&self, script_hash: &ScriptHash) -> Arc<Script>;
}
