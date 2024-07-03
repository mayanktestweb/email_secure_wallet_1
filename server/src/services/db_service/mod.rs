use std::{fs::File, io::{Error, Read}, path::Path};

use serde_json::Value;



pub trait IDBSerive {
    fn store_json_data(&self, data: Value) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DummyDBService;


impl IDBSerive for DummyDBService {
    fn store_json_data(&self, data: Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:#?}", data);
        Ok(())
    }
}