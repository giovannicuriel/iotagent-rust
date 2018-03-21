use std::fs::File;
use std::io::prelude::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub enum BrokerType {
    #[serde(rename = "orion")]
    Orion,
    #[serde(rename = "Kafka")]
    Kafka,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TlsVersion {
    #[serde(rename = "TLSv1_2_method")]
    Tlsv12Method,
    #[serde(rename = "TLSv1_method")]
    Tlsv1Method,
    #[serde(rename = "DTLSv1_method")]
    Dtlsv1Method,
    #[serde(rename = "DTLSv1_2_method")]
    Dtlsv12Method,
}

// TLS options in configuration file.
#[derive(Serialize, Deserialize, Debug)]
pub struct TlsOptions {
    // File where the private key is.
    // If TLS is used, this attribute is mandatory.
    pub key: String,
    // File where the certificate is.
    // If TLS is used, this attribute is mandatory.
    pub cert: String,
    // File where CA (certification authority) certificate is.
    // If TLS is used, this attribute is mandatory.
    pub ca: Vec<String>,
    // TLS version.
    // If TLS is used, this attribute is mandatory.
    pub version: TlsVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttOptions {
    // MQTT broker address (this attribute doesn't include port).
    pub host: String,
    // MQTT broker port
    pub port: u16,
    // Protocol ID.
    // Default value is MQIsdp
    #[serde(rename = "protocolId", default = "default_protocol_id")]
    pub protocol_id: String,
    // Protocol version.
    // Default value is 3
    #[serde(rename = "protocolVersion", default = "default_protocol_version")]
    pub protocol_version: u8,
    // Flag indicating whether traffic must be encrypted.
    pub secure: bool,
    // Encription configuration.
    // This attribute will be used only if 'secure' is true.
    #[serde(default = "default_tls_options")]
    pub tls: Option<TlsOptions>,
}
fn default_protocol_id() -> String {
    "MQIsdp".to_string()
}
fn default_protocol_version() -> u8 {
    3
}
fn default_tls_options() -> Option<TlsOptions> {
    Option::None
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BrokerOptions {
    // Broker address.
    // This attribute includes port, such as localhost:9092.
    pub host: String,

    // Broker type.
    // Default value is "orion".
    #[serde(default = "default_broker_type")]
    pub broker_type: BrokerType,
}
fn default_broker_type() -> BrokerType {
    BrokerType::Orion
}

// Kafka options for device manager
#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaOptions {
    #[serde(rename = "autoCommit", default = "default_kafka_auto_commit")]
    pub auto_commit: bool,
    #[serde(rename = "fetchMaxWaitMs", default = "default_kafka_fetch_max_wait")]
    pub fetch_max_wait_ms: u32,
    #[serde(rename = "fetchMaxBytes", default = "default_kafka_fetch_max_bytes")]
    pub fetch_max_bytes: u32,
    #[serde(rename = "groupId", default = "default_kafka_options_group_id")]
    pub group_id: String,
}

fn default_kafka_auto_commit() -> bool {
    true
}
fn default_kafka_fetch_max_wait() -> u32 {
    1000
}
fn default_kafka_fetch_max_bytes() -> u32 {
    10485756
}
fn default_kafka_options_group_id() -> String {
    "iotagent".to_string()
}

// Kafka topic structure
#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaTopic {
    pub topic: String,
}

// Device manager options
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceManagerOptions {
    // Device manager address.
    // This attribute includes port, such as localhost:4000.
    #[serde(rename = "kafkaHost", default = "default_kafka_host")]
    pub kafka_host: String,

    // Kafka options
    #[serde(rename = "kafkaOptions")]
    pub kafka_options: KafkaOptions,
    // Kafka topics that iotagent will subscribe
    #[serde(rename = "kafkaTopics")]
    pub kakfa_topics: Vec<KafkaTopic>,
}

fn default_kafka_host() -> String {
    "zookeeper:2181".to_string()
}

// This is the main structure for configuration data.
// It uses de SERDE (serialization-deserialization) functions
// from serde_json crate. It annotates all attributes that should
// be deserialized with a default value. Problem is that all
// these values should come from a function call, not the actual
// value.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigOptions {
    // MQTT options.
    pub mqtt: MqttOptions,
    // Context broker options.
    pub broker: BrokerOptions,
    // Device manager options.
    pub device_manager: DeviceManagerOptions,
}

// Function to read a file and fill the configuration data structure.
pub fn read_config(config_file: &str) -> ConfigOptions {
    let mut f = File::open(config_file).expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect(
        "something went wrong reading the file",
    );


    // This is the 'return' part. Note that it doesn't containt a ';'
    // It will build a new Config object (as defined by the return value of this function)
    // and use the data returned from the unwrap() - actually from the 'from_str' - method.
    serde_json::from_str(&content).unwrap()
}
