// use config;
// use data_broker;
// use serde_json::{Value, Map};

// pub struct OrionHandler {
//     host: String
// }

// impl OrionHandler {
//     pub fn new(cfg: config::ConfigOptions) -> OrionHandler {
//         return OrionHandler {
//             host: cfg.broker.host.clone()
//         }
//     }
// }

// impl data_broker::DataBroker for OrionHandler {
//     fn update_data(&self, service: String, device_id: String, attributes: Map<String, Value>) {
//         println!("Sending update data to somewhere");
//     }
// }
