use std::error::Error;

use my_mac::{
    installers::InstallationManager,
    models::{Installation, InstallationStep, InstallationStepAction},
};

fn main() -> Result<(), Box<dyn Error>> {
    let system_installation =
        Installation::new("System Components", "Essentials for MacBook").with_install_steps(
            vec![
                InstallationStep::new(
                    "Homebrew",
                    "The Missing Package Manager for macOS (or Linux)",
                    InstallationStepAction::InternetScriptInstall("brew".to_string(), "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh".to_string()),
                ),
                InstallationStep::new(
                    "Docker",
                    "Docker helps developers build, share, run, and verify applications anywhere — without tedious environment configuration or management.",
                    InstallationStepAction::InternetScriptInstall("docker".to_string(), "https://get.docker.com".to_string()),
                ),
            ],
        );
    let karabiner_installation = Installation::new(
        "Karabiner",
        "",
    ).with_install_steps(vec![
        InstallationStep::new("Karabiner-Elements", "Keyboard customize manager", InstallationStepAction::BrewFormulaInstall("karabiner-elements".to_string())),
        InstallationStep::new(
            "Karabiner: Vim Mode Plus",
            "A complex modification for Karabiner Elements that mimics Vim's navigation throughout your entire Mac.",
            InstallationStepAction::BrowserOpen("karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/vim.json".to_string(), true),
        ),
        InstallationStep::new(
            "Karabiner: Chinese English",
            "A switch of Chinese & English",
            InstallationStepAction::BrowserOpen("karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/chinese-input.json".to_string(), true),
        ),
    ]);
    let other_installation =
        Installation::new("Other components", "").with_install_steps(vec![
            InstallationStep::new(
                "Amethyst",
                "Till window manager",
                InstallationStepAction::BrewFormulaInstall("amethyst".to_string()),
            ),
            InstallationStep::new(
                "Vimlike",
                "Safari extension for vimlike operate experience",
                InstallationStepAction::AppleStoreOpen("vimlike/id1584519802".to_string()),
            ),
            InstallationStep::new(
                "Itsycal",
                "Tiny menu bar calendar.",
                InstallationStepAction::BrewFormulaInstall("itsycal".to_string()),
            ),
        ]);

    let _ = InstallationManager::new()
        .add_installations(vec![
            system_installation,
            karabiner_installation,
            other_installation,
        ])
        .install();

    Ok(())
}
