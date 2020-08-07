use std::time::{SystemTime, UNIX_EPOCH};

pub fn random_name_for_layer() -> String {
  let start = SystemTime::now();
  let elapsed = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
  format!("layer-{}", elapsed).to_string()
}