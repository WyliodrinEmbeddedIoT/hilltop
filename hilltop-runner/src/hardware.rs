use std::path::{Path, PathBuf};

use tracing::warn;

use crate::{
    config::{RunnerConfig, devices::HilltopDeviceDescriptor, hardware::HardwareConfig},
    job::error::HardwareError,
};

pub struct HardwareManager {
    devices: Vec<HardwareDevice>,
    configurations: Vec<HardwareConfig>,
}

impl HardwareManager {
    pub async fn new(runner_config: &RunnerConfig) -> anyhow::Result<Self> {
        let mut devices: Vec<HardwareDevice> = Vec::new();

        let host_devices = nusb::list_devices().await?.collect::<Vec<_>>();
        tracing::info!("Found {} USB devices on host", host_devices.len());
        for (idx, host_device) in host_devices.iter().enumerate() {
            tracing::debug!("Usb device #{idx}: {host_device:#?}");
        }

        for device_descriptor in &runner_config.devices {
            tracing::info!("Registering device: {}", device_descriptor.device_name);

            // Ensure device was not previously registered
            if devices
                .iter()
                .any(|d| d.descriptor.matches_name(device_descriptor))
            {
                anyhow::bail!(
                    "Cannot register device '{}': already registered",
                    device_descriptor.device_name
                );
            };

            if devices
                .iter()
                .any(|d| d.descriptor.matches_usb_info(device_descriptor))
            {
                anyhow::bail!(
                    "Cannot register device '{}': another device with same USB info already registered",
                    device_descriptor.device_name
                );
            };

            let device_info = host_devices.iter().find(|dev| {
                dev.product_id() == device_descriptor.product_id_u16().unwrap_or(0)
                    && dev.vendor_id() == device_descriptor.vendor_id_u16().unwrap_or(0)
                    && dev.serial_number().unwrap_or("") == device_descriptor.serial
            });
            // Device not plugged in, skip it instead of crashing the runner
            if device_info.is_none() {
                warn!(
                    "Device '{}' not found, skipping",
                    device_descriptor.device_name
                );
                continue;
            }

            let device =
                HardwareDevice::new(device_descriptor.clone(), device_info.unwrap().clone())?;
            devices.push(device);
        }
        let mut configurations: Vec<HardwareConfig> = Vec::new();

        for hw_config in runner_config.hardware_configurations.iter() {
            // Configs needing a missing device get dropped, not the whole runner

            tracing::info!("Validating hardware config '{}'", hw_config.config_name);

            let mut all_devices_available = true; //

            for device_ref in hw_config.devices.iter() {
                let device_found = devices
                    .iter()
                    .any(|d| d.descriptor.device_name == device_ref.device_name);
                // Skip this config
                if !device_found {
                    warn!(
                        "Hardware config '{}' references unknown device '{}', skipping config",
                        hw_config.config_name, device_ref.device_name
                    );
                    all_devices_available = false;
                    break;
                }

                let is_passthrough = device_ref.device_passthrough == Some(true);
                match (is_passthrough, &device_ref.container_path) {
                    (true, Some(_)) => anyhow::bail!(
                        "Hardware config '{}', device '{}': device_passthrough and container_path are mutually exclusive",
                        hw_config.config_name,
                        device_ref.device_name
                    ),
                    (false, None) => anyhow::bail!(
                        "Hardware config '{}', device '{}': container_path is required when device_passthrough is not set",
                        hw_config.config_name,
                        device_ref.device_name
                    ),
                    _ => {}
                }
            }

            if all_devices_available {
                configurations.push(hw_config.clone());
            }
        }

        Ok(Self {
            devices,
            configurations,
        })
    }

    pub fn get_configuration(&self, name: &str) -> Option<&HardwareConfig> {
        self.configurations
            .iter()
            .find(|cfg| cfg.config_name == name)
    }

    pub fn get_device(&self, name: &str) -> Option<&HardwareDevice> {
        self.devices
            .iter()
            .find(|d| d.descriptor.device_name == name)
    }

    pub fn get_device_mut(&mut self, name: &str) -> Option<&mut HardwareDevice> {
        self.devices
            .iter_mut()
            .find(|d| d.descriptor.device_name == name)
    }

    pub fn can_acquire_configuration(&self, name: &str) -> Result<bool, HardwareError> {
        let config = self
            .get_configuration(name)
            .ok_or(HardwareError::UnknownConfiguration(name.to_string()))?;

        for device_ref in config.devices.iter() {
            let device = self.get_device(&device_ref.device_name).ok_or_else(|| {
                tracing::error!(
                    "Hardware config '{}' references unknown device '{}' despite validation",
                    name,
                    device_ref.device_name
                );
                HardwareError::InternalError
            })?;

            if device.in_use {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn acquire_configuration(&mut self, name: &str) -> Result<(), HardwareError> {
        // TODO: find way to get rid of clone :(
        let config = self
            .get_configuration(name)
            .ok_or(HardwareError::UnknownConfiguration(name.to_string()))?
            .clone();

        for device_ref in config.devices.iter() {
            let device = self
                .get_device_mut(&device_ref.device_name)
                .ok_or_else(|| {
                    tracing::error!(
                        "Hardware config '{}' references unknown device '{}' despite validation",
                        name,
                        device_ref.device_name
                    );
                    HardwareError::InternalError
                })?;

            if device.in_use {
                return Err(HardwareError::ResourcesBusy);
            } else {
                device.in_use = true;
            }
        }

        Ok(())
    }

    pub fn release_configuration(&mut self, name: &str) -> Result<(), HardwareError> {
        let config = self
            .get_configuration(name)
            .ok_or(HardwareError::UnknownConfiguration(name.to_string()))?
            .clone();

        for device_ref in config.devices.iter() {
            let device = self
                .get_device_mut(&device_ref.device_name)
                .ok_or_else(|| {
                    tracing::error!(
                        "Hardware config '{}' references unknown device '{}' despite validation",
                        name,
                        device_ref.device_name
                    );
                    HardwareError::InternalError
                })?;

            device.in_use = false;
        }

        Ok(())
    }
}

pub struct HardwareDevice {
    descriptor: HilltopDeviceDescriptor,
    dev_path: Option<PathBuf>,
    bus_path: PathBuf,
    in_use: bool,
}

impl HardwareDevice {
    pub fn new(
        descriptor: HilltopDeviceDescriptor,
        device_info: nusb::DeviceInfo,
    ) -> anyhow::Result<Self> {
        let bus_path = PathBuf::from(format!(
            "/dev/bus/usb/{}/{:03}",
            device_info.bus_id(),
            device_info.device_address()
        ));

        let sys_path = device_info.sysfs_path().canonicalize().unwrap();

        let mut dev_path: Option<PathBuf> = None;

        for entry in std::fs::read_dir(Path::new("/sys/class/tty"))? {
            let entry = entry?;
            let symlink_path = entry.path();

            // Resolve the symlink to an absolute sysfs path
            let resolved = symlink_path.canonicalize()?;

            // Check if the resolved sysfs path is under our target device path
            if resolved.starts_with(&sys_path) {
                // The /dev/ node has the same name as the tty class entry
                let dev_name = symlink_path.file_name().unwrap();
                dev_path = Some(Path::new("/dev").join(dev_name));
            }
        }

        Ok(Self {
            descriptor,
            dev_path,
            bus_path,
            in_use: false,
        })
    }

    pub fn dev_path(&self) -> Option<&Path> {
        self.dev_path.as_deref()
    }

    pub fn bus_path(&self) -> &Path {
        &self.bus_path
    }

    pub fn device_name(&self) -> &str {
        &self.descriptor.device_name
    }

    pub fn serial(&self) -> &str {
        &self.descriptor.serial
    }

    pub fn probe_rs_chip(&self) -> Option<&str> {
        self.descriptor.probe_rs_chip.as_deref()
    }

    pub fn tockloader_board(&self) -> Option<&str> {
        self.descriptor.tockloader_board.as_deref()
    }

    /// Return the probe-rs selector for this configured USB device.
    ///
    /// probe-rs accepts selectors in VID:PID:SERIAL form. The serial comes
    /// from runner.json, so jobs do not need to guess an enumeration index.
    pub fn probe_selector(&self) -> Option<String> {
        Some(format!(
            "{:04x}:{:04x}:{}",
            self.descriptor.vendor_id_u16()?,
            self.descriptor.product_id_u16()?,
            self.descriptor.serial
        ))
    }
}
