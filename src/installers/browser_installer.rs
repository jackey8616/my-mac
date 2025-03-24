use std::process::Command;

use crate::traits::Installer;

pub struct BrowserInstaller {
    name: String,
    description: String,
    url: String,
}

impl BrowserInstaller {
    pub fn new(name: &str, description: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

impl Installer for BrowserInstaller {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn is_installed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(false)
    }

    fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("open")
            .args(&["-a", "Safari", &self.url])
            .status()?;
        Ok(())
    }
}
