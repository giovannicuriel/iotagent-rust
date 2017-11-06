use std::fs::File;
use std::io::prelude::*;
use serde_json;



// This is the main structure for configuration data.
// It uses de SERDE (serialization-deserialization) functions
// from serde_json crate. It annotates all attributes that should
// be deserialized with a default value. Problem is that all
// these values should come from a function call, not the actual
// value.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_broker_address")]
    pub broker_address: String,
    #[serde(default = "default_broker_port")]
    pub broker_port: u16,
    #[serde(default = "default_tls")]
    pub tls: bool,
    #[serde(default = "default_keep_alive")]
    pub keep_alive: u16,
    #[serde(default = "default_reconnect")]
    pub reconnect: u16
}
fn default_tls() -> bool { false }
fn default_broker_port() -> u16 { 1883 }
fn default_broker_address() -> String { "localhost".to_string() }
fn default_keep_alive() -> u16 { 3 }
fn default_reconnect() -> u16 { 5 }

// Function to read a file and fill the configuration data structure.
pub fn read(config_file: &str) -> Config {
    let mut f = File::open(config_file).expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("something went wrong reading the file");


    // This is the 'return' part. Note that it doesn't containt a ';'
    // It will build a new Config object (as defined by the return value of this function)
    // and use the data returned from the unwrap() - actually from the 'from_str' - method.
    serde_json::from_str(&content).unwrap()
}
