use crate::config::devices::HilltopDeviceDescriptor;
use crate::config::hardware::HardwareConfig;
use crate::config::image::ImageConfig;

pub mod devices;
pub mod hardware;
pub mod image;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunnerConfig {
    pub runner_slug: String,
    pub runner_api_key_path: String,
    pub broker_url: String,
    pub broker_socket_url: String,
    pub available_images: Vec<ImageConfig>,
    pub devices: Vec<HilltopDeviceDescriptor>,
    pub hardware_configurations: Vec<HardwareConfig>,
}

impl RunnerConfig {
    /// Load runner config from JSON string
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| anyhow::anyhow!("Failed to parse runner config: {}", e))
    }

    /// Get image config by name
    pub fn get_image(&self, name: &str) -> Option<&ImageConfig> {
        self.available_images.iter().find(|img| img.name == name)
    }

    /// Get hardware config by name
    pub fn get_hardware(&self, name: &str) -> Option<&HardwareConfig> {
        self.hardware_configurations
            .iter()
            .find(|hw| hw.config_name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let json = r#"{
            "available_images": [
                {"name": "ubuntu:latest", "image": "./images/Dockerfile.ubuntu", "tag": "hilltop-ubuntu:latest"}
            ],
            "hardware_configurations": [
                {"name": "dummy"}
            ]
        }"#;
        let config = RunnerConfig::from_json(json).unwrap();
        assert_eq!(config.available_images.len(), 1);
        assert_eq!(config.hardware_configurations.len(), 1);
        assert!(config.get_image("ubuntu:latest").is_some());
        assert!(config.get_image("nonexistent").is_none());
        assert!(config.get_hardware("dummy").is_some());
        assert!(config.get_hardware("nonexistent").is_none());
    }
}
