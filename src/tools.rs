// pub fn flatten_object(path: String, v: Map<String, Value>) -> Vec<(String, Value)> {
//     let mut ret: Vec<(String, Value)> = Vec::new();
//     for pair in v {
//         let mut separator: String = "".into();

//         if path != "" {
//             separator = ".".into();
//         }

//         // Only 'path' should be cloned because it will generate another object
//         // If path was used, the next loop would access a moved value.
//         let current_path: String = path.clone() + &separator + &pair.0;
//         match pair.1 {
//             Value::Bool(_) => {
//                 ret.push((current_path, pair.1));
//             }
//             Value::Number(_) => {
//                 ret.push((current_path, pair.1));
//             }
//             Value::String(_) => {
//                 ret.push((current_path, pair.1));
//             }
//             Value::Array(_) => {
//                 ret.push((current_path, pair.1));
//             }
//             Value::Object(v) => {
//                 ret.extend(MqttContext::flatten_object(current_path, v));
//             }
//             _ => {
//                 // Null
//             }
//         }
//     }

//     ret
//     }