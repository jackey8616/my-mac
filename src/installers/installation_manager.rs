use std::{
    error::Error,
    io::{self, Read, Write},
    process::Command,
};

use crate::{
    models::{installation::Installation, installation_step_action::InstallationStepAction},
    traits::Installer,
};

use super::{AppleStoreInstaller, BrewFormulaInstaller, InternetScriptInstaller};

pub struct InstallationManager {
    pub installations: Vec<Installation>,
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
        let _ = self.installations.iter().for_each(|installation| {
            println!("Installing {}", installation.name);
            let steps = installation.clone().install_steps;
            for step in steps {
                match step.action {
                    InstallationStepAction::BrowserOpen(url, wait_user_confirm) => {
                        println!("Opening {} url: {}", step.name, url);
                        let _ = Command::new("open").args(&["-a", "Safari", &url]).status();
                        if wait_user_confirm {
                            print!("Is opened? [Y]");
                            io::stdout().flush().unwrap();

                            loop {
                                let mut buffer = [0; 1];
                                io::stdin().read_exact(&mut buffer).expect("Read failed");
                                let input = buffer[0] as char;
                                if input == 'y' || input == 'Y' {
                                    break;
                                } else {
                                }
                            }
                        }
                    }
                    InstallationStepAction::InternetScriptInstall(name, script) => {
                        let _ = InternetScriptInstaller::new(&name, &step.description, &script)
                            .install();
                    }
                    InstallationStepAction::BrewFormulaInstall(formula_name) => {
                        let _ =
                            BrewFormulaInstaller::new(&formula_name, &step.description).install();
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
