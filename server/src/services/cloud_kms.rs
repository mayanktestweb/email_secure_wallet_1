use crate::utils;



pub trait ICloudKeyManagementService {
    fn encrypt_credentials(&self, private_key_bytes: &Vec<u8>) -> Option<String>;
}

pub struct DummyCloudKeyManager;

impl ICloudKeyManagementService for DummyCloudKeyManager {
    fn encrypt_credentials(&self, private_key_bytes: &Vec<u8>) -> Option<String> {
        utils::bytes_to_string(private_key_bytes)
    }
}