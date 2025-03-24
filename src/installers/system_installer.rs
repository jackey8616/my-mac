use std::{
    error::Error,
    fs,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::traits::{Downloader, Installer, ScriptExecutor};

pub struct SystemInstaller {
    downloader: Box<dyn Downloader>,
    executor: Box<dyn ScriptExecutor>,
}

impl SystemInstaller {
    pub fn new(downloader: Box<dyn Downloader>, executor: Box<dyn ScriptExecutor>) -> Self {
        Self {
            downloader,
            executor,
        }
    }

    fn install(&self, name: &str, url: &str, path: &str) -> Result<bool, Box<dyn Error>> {
        print!("Downloading {} installation script...", name);
        match self.downloader.download(url, path) {
            Ok(_) => (),
            Err(error) => return Err(error.into()),
        };

        Command::new("chmod").args(&["+x", path]).status()?;

        print!("Executing {} installation script...", name);

        let success = self.executor.execute(path)?;
        let _ = fs::remove_file(path);
        if success {
            println!("{} {}", name, "installed successfully!".green().bold());
            return Ok(true);
        } else {
            return Err(format!("{} {}!", "Failed to install".red().bold(), name).into());
        }
    }
}

impl Installer for SystemInstaller {
    fn name(&self) -> &str {
        "system"
    }

    fn description(&self) -> &str {
        "System installed packages"
    }

    fn is_installed(&self) -> Result<bool, Box<dyn Error>> {
        let brew_installed = Command::new("which")
            .arg("brew")
            .stdout(Stdio::null())
            .status()?
            .success();
        let docker_installed = Command::new("which")
            .arg("docker")
            .stdout(Stdio::null())
            .status()?
            .success();
        Ok(brew_installed && docker_installed)
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        if self.is_installed()? {
            println!("{}", "Homebrew & Docker installed".green().bold());
            return Ok(());
        }

        let _brew_install_result = match self.install(
            "Homebrew",
            "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
            "./tmp/homebrew_install.sh",
        ) {
            Ok(_) => (),
            Err(error) => panic!("{:?}", error),
        };

        let _docker_install_result = match self.install(
            "Docker",
            "https://get.docker.com",
            "./tmp/docker_install.sh",
        ) {
            Ok(_) => (),
            Err(error) => panic!("{:?}", error),
        };

        Ok(())
    }
}
