#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareConfig {
    pub config_name: String,
    pub devices: Vec<DeviceReference>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceReference {
    pub device_name: String,
    pub container_path: Option<String>,
    pub device_passthrough: Option<bool>,
}
