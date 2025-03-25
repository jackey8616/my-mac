use crate::traits::Installer;
use colored::Colorize;
use std::{
    error::Error,
    process::{Command, Stdio},
};

pub struct AppleStoreInstaller {
    name: String,
    description: String,
    bundle_id: String,
}

impl AppleStoreInstaller {
    pub fn new(name: &str, description: &str, bundle_id: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            bundle_id: bundle_id.to_string(),
        }
    }
}

impl Installer for AppleStoreInstaller {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn is_installed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let output = Command::new("ls")
            .args(&["/Applications", "|", "grep", "-i", &self.name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()?;

        Ok(String::from_utf8(output.stdout)?.len() > 0)
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        if self.is_installed()? {
            println!("{} is {}", self.name, "installed".green());
            return Ok(());
        }

        println!("Opening AppleStore page to install {}", self.name);
        let url = format!("macappstore://apps.apple.com/us/app/{}", self.bundle_id);
        Command::new("open").arg(url);
        Ok(())
    }
}

#[cfg(test)]
mod apple_store_installer_tests {
    use super::*;

    #[test]
    fn test_initializer() {
        let installer = AppleStoreInstaller::new(
            "Test App", 
            "A test application", 
            "test-app-bundle-id"
        );

        assert_eq!(installer.name(), "Test App");
        assert_eq!(installer.description(), "A test application");
    }
}
