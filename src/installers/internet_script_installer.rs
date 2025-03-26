use std::{
    fs,
    process::{Command, Stdio},
};

use crate::{
    core::{CommandExecutor, HttpDownloader},
    traits::{Downloader, Executor, Installer},
};

use colored::Colorize;

pub struct InternetScriptInstaller {
    name: String,
    description: String,
    url: String,
}

impl InternetScriptInstaller {
    pub fn new(name: &str, description: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

impl Installer for InternetScriptInstaller {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn is_installed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let brew_installed = Command::new("which")
            .arg(&self.name)
            .stdout(Stdio::null())
            .status()?
            .success();
        Ok(brew_installed)
    }

    fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_installed()? {
            println!("{} {}", self.name, "installed".green().bold());
            return Ok(());
        }

        print!("Downloading {} installation script...", self.name);
        let downloader: Box<dyn Downloader> = Box::new(HttpDownloader);
        let path = format!("./tmp/{}_install.sh", self.name);
        match downloader.download(&self.url, path.as_str()) {
            Ok(_) => (),
            Err(error) => return Err(error.into()),
        };

        Command::new("chmod")
            .args(&["+x", path.as_str()])
            .status()?;

        print!("Executing {} installation script...", self.name);

        let output = CommandExecutor::new("bash".to_string(), vec![path.clone()]).execute();
        let _ = fs::remove_file(path);
        if output.is_ok() {
            println!("{} {}", self.name, "installed successfully!".green().bold());
            return Ok(());
        } else {
            return Err(format!("{} {}!", "Failed to install".red().bold(), self.name).into());
        }
    }
}

#[cfg(test)]
mod internet_script_installer_tests {
    use super::*;

    #[test]
    fn test_initializer() {
        let installer = InternetScriptInstaller::new(
            "Test Internet Script",
            "A test installation script",
            "https://invalid.example.com/non-existent-script.sh",
        );

        assert_eq!(installer.name(), "Test Internet Script");
        assert_eq!(installer.description(), "A test installation script");
    }
}
