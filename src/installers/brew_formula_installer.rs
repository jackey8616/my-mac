use std::{
    error::Error,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::traits::{Executor, Installer};

pub struct BrewFormulaInstaller<C: Executor<String>> {
    formula_name: String,
    description: String,
    _info: String,
    _executor: C,
}

impl<C: Executor<String>> BrewFormulaInstaller<C> {
    pub fn new(formula_name: &str, description: &str, executor: C) -> Self {
        match executor.execute() {
            Ok(info) => Self {
                formula_name: formula_name.to_string(),
                description: description.to_string(),
                _info: info,
                _executor: executor,
            },
            Err(error) => panic!("{}", error),
        }
    }

    fn is_cask(&self) -> bool {
        self._info
            .contains("From: https://github.com/Homebrew/homebrew-cask/blob/HEAD/Casks")
    }

    fn install_via_brew(&self) -> Result<(), Box<dyn Error>> {
        let args = if self.is_cask() {
            ["install", "--cask", &self.formula_name]
        } else {
            ["install", &self.formula_name, ""]
        };

        let status = Command::new("brew")
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if status.success() {
            println!(
                "{} {}",
                self.formula_name.green().bold(),
                "install successfully!".green().bold()
            );
        } else {
            println!(
                "{} {}",
                "Failed to install".red().bold(),
                self.formula_name.red().bold()
            );
        }

        Ok(())
    }
}

impl<C: Executor<String>> Installer for BrewFormulaInstaller<C> {
    fn name(&self) -> &str {
        &self.formula_name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn is_installed(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self._info.contains("Installed"))
    }

    fn install(&self) -> Result<(), Box<dyn Error>> {
        if self.is_installed()? {
            println!("{} is {}", self.formula_name, "installed".green().bold());
            return Ok(());
        }

        println!("Installing {} vis Homebrew...", self.formula_name);
        self.install_via_brew()
    }
}

#[cfg(test)]
mod brew_formula_installer_tests {
    use super::*;

    #[test]
    fn test_initializer() {
        struct MockExecutor {}

        impl MockExecutor {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl Executor<String> for MockExecutor {
            fn execute(&self) -> Result<String, Box<dyn Error>> {
                Ok("Executed".to_string())
            }
        }

        let installer = BrewFormulaInstaller::new(
            "wget",
            "GNU Wget is a file and recursive website downloader",
            MockExecutor::new(),
        );

        assert_eq!(installer.name(), "wget");
        assert_eq!(
            installer.description(),
            "GNU Wget is a file and recursive website downloader"
        );
    }
}
