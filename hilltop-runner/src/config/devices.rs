#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HilltopDeviceDescriptor {
    pub device_name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub serial: String,
    // no board/tool fields here on purpose - that config comes from
    // job.json's env now, not the runner
}

impl HilltopDeviceDescriptor {
    /// Convert vendor_id from hex string to u16. Returns None if parsing fails.
    pub fn vendor_id_u16(&self) -> Option<u16> {
        let stripped_prefix = self.vendor_id.trim_start_matches("0x");
        u16::from_str_radix(stripped_prefix, 16).ok()
    }

    /// Convert product_id from hex string to u16. Returns None if parsing fails.
    pub fn product_id_u16(&self) -> Option<u16> {
        let stripped_prefix = self.product_id.trim_start_matches("0x");
        u16::from_str_radix(stripped_prefix, 16).ok()
    }

    pub fn matches_usb_info(&self, other: &Self) -> bool {
        self.vendor_id_u16() == other.vendor_id_u16()
            && self.product_id_u16() == other.product_id_u16()
            && self.serial == other.serial
    }

    pub fn matches_name(&self, other: &Self) -> bool {
        self.device_name == other.device_name
    }
}
