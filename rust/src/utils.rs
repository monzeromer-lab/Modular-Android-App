use std::collections::HashMap;

pub fn process_input_data(input: &str) -> String {
    log::info!("Utils: Processing input data: {}", input);
    
    // Simple data processing example
    let processed = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                }
            } else {
                c
            }
        })
        .collect::<String>();
    
    format!("Processed: {}", processed)
}

pub fn calculate_hash(data: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn validate_input(input: &str) -> bool {
    !input.is_empty() && input.len() <= 1000
}

pub fn create_response_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("version".to_string(), "1.0.0".to_string());
    map.insert("status".to_string(), "ok".to_string());
    map.insert("timestamp".to_string(), format!("{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()));
    map
}

pub fn format_response(data: &str, metadata: &HashMap<String, String>) -> String {
    let mut response = format!("Data: {}", data);
    for (key, value) in metadata {
        response.push_str(&format!("\n{}: {}", key, value));
    }
    response
} 