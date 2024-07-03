/**
 * This module is reponsible for all signature related services
 * Like generating signature for a message
 */


use alloy::signers::{k256::{ecdsa::{Signature, SigningKey}, Secp256k1}, local::{LocalSigner, PrivateKeySigner}};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{cloud_kms::ICloudKeyManagementService, db_service::{self, IDBSerive}};



pub struct MsgSignature {
    pub msg: String,
    pub signature: Signature
}


pub fn generate_signers(num_of_signers: i32) -> Option<Vec<LocalSigner<SigningKey>>> {
    let mut signers = vec![];

    for i in 0..num_of_signers {
        signers.push(LocalSigner::random());
    }

    Some(signers)
}

pub fn store_signers(
    signers: Vec<LocalSigner<SigningKey>>, 
    cloud_kms_service_provider: Box<dyn ICloudKeyManagementService>,
    db_service_provider: Box<dyn IDBSerive>
) -> Result<(), Box<dyn std::error::Error>> {
    for (index, signer) in signers.iter().enumerate() {
        let credential = signer.credential().to_bytes();
        let mut pkey = vec![];

        for b in credential {
            pkey.push(b);
        }

        let encrypted_credential = cloud_kms_service_provider.encrypt_credentials(&pkey).unwrap();
        db_service_provider.store_json_data(json!({"key_id": index+1, "value": encrypted_credential}));
    }
    Ok(())
}

