use serde_json::{Value, Map};

// This is the trait that should be implemented for a data broker
pub trait DataBroker {
    fn update_data(&self, service: String, device_id: String, attributes: Map<String, Value>);
}
