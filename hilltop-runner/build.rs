fn main() {
    let enabled_apis: Vec<String> = std::env::vars()
        .filter(|(key, _)| key.starts_with("CARGO_FEATURE_API_"))
        .map(|(key, _)| key)
        .collect();

    if enabled_apis.is_empty() {
        panic!("At least one API version feature must be selected");
    }

    if enabled_apis.len() > 1 {
        panic!(
            "Only one API version feature can be selected at a time. Found: {:?}",
            enabled_apis
        );
    }
}
