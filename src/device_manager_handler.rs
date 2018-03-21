
use config;
use std::collections::HashMap;

// Device attribute interface
#[derive(Deserialize, Debug)]
pub struct DeviceAttribute {
    // Label ID
    id: u32,
    // Attribute label - defined by user
    label: String,
    // Attribute type (static, dynamic, meta)
    #[serde(rename="type")]
    attribute_type: String,
    // Value type (float, integer, geo, string)
    value_type: String,
}

#[derive(Deserialize, Debug)]
pub struct DeviceMeta {
    // Tenant associated to this device - used in device creation
    service: Option<String>,
    // MQTT topic on which a message will be sent - used in device configuration
    topic: Option<String>,
    // No clue what this is for - used in device configuration
    id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceData {
    id: String,
    attrs: Option<HashMap<String, Vec<DeviceAttribute>>>,
}

// Device event message
#[derive(Deserialize, Debug)]
pub struct DeviceEvent {
    // Event - "create", "update", "remove", "configure"
    event: String,
    // Meta attributes related to this device
    meta: DeviceMeta,
    // Attributes and other stuff related to this device
    data: DeviceData,
}

// Device manager handler.
// This class is responsible to wrap all functions needed from device manager.
pub struct DeviceManagerHandler {
    // There should be a KafkaHandler or something here.
}

impl DeviceManagerHandler {
    // Constructor.
    pub fn new(config: config::ConfigOptions) -> DeviceManagerHandler {
        DeviceManagerHandler {}
    }
}
