use std::{
    error::Error,
    io::{self, Read, Write},
    process::Command,
};

use crate::{
    core::CommandExecutor,
    models::{Installation, InstallationStepAction},
    traits::Installer,
};

use super::{AppleStoreInstaller, BrewFormulaInstaller, InternetScriptInstaller};

pub struct InstallationManager {
    pub installations: Vec<Installation>,
}

impl Default for InstallationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InstallationManager {
    pub fn new() -> Self {
        Self {
            installations: Vec::new(),
        }
    }

    pub fn add_installations(mut self, installations: Vec<Installation>) -> Self {
        self.installations = installations;
        self
    }

    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        self.installations.iter().for_each(|installation| {
            println!("Installing {}", installation.name);
            let steps = installation.clone().install_steps;
            for step in steps {
                match step.action {
                    InstallationStepAction::BrowserOpen(url, wait_user_confirm) => {
                        println!("Opening {} url: {}", step.name, url);
                        let _ = Command::new("open").args(["-a", "Safari", &url]).status();
                        if wait_user_confirm {
                            print!("Is opened? [Y]");
                            io::stdout().flush().unwrap();

                            loop {
                                let mut buffer = [0; 1];
                                io::stdin().read_exact(&mut buffer).expect("Read failed");
                                let input = buffer[0] as char;
                                if input == 'y' || input == 'Y' {
                                    break;
                                }
                            }
                        }
                    }
                    InstallationStepAction::InternetScriptInstall(name, script) => {
                        let _ = InternetScriptInstaller::new(&name, &step.description, &script)
                            .install();
                    }
                    InstallationStepAction::BrewFormulaInstall(formula_name) => {
                        let executor = CommandExecutor::new(
                            "brew".to_string(),
                            vec!["info".to_string(), formula_name.clone()],
                        );
                        let _ =
                            BrewFormulaInstaller::new(&formula_name, &step.description, executor)
                                .install();
                    }
                    InstallationStepAction::AppleStoreOpen(bundle_id) => {
                        let _ = AppleStoreInstaller::new(&step.name, &step.description, &bundle_id)
                            .install();
                    }
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod installation_manager_tests {
    use crate::{
        installers::InstallationManager,
        models::{Installation, InstallationStep, InstallationStepAction},
    };

    #[test]
    fn test_initializer() {
        let mock_installation = Installation::new("System Test", "Test system components", true)
            .with_install_steps(vec![InstallationStep::new(
                "Test Homebrew",
                "Test Homebrew installation",
                InstallationStepAction::InternetScriptInstall(
                    "brew".to_string(),
                    "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
                        .to_string(),
                ),
            )]);

        let manager = InstallationManager::new().add_installations(vec![mock_installation]);

        assert_eq!(manager.installations.len(), 1);
        assert_eq!(manager.installations[0].install_steps.len(), 1);
    }
}
