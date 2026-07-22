#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageConfig {
    pub name: String,
    /// Relative path to the image file from the config directory (or directory
    /// in which runner configuration exists)
    pub image: String,
    pub tag: String,
}
