use my_mac::{
    core::{BashExecutor, HttpDownloader},
    installers::{AppleStoreInstaller, BrewFormulaInstaller, BrowserInstaller, SystemInstaller},
    traits::Installer,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let downloader = Box::new(HttpDownloader);
    let bash_executor = Box::new(BashExecutor);
    let system_installer = SystemInstaller::new(downloader, bash_executor);
    match system_installer.install() {
        Ok(_) => (),
        Err(error) => return Err(error.into()),
    };

    let _ = match BrewFormulaInstaller::new("karabiner-elements", "Keyboard customize manager") {
        Ok(installer) => installer.install(),
        Err(error) => return Err(error.into()),
    };
    let _ = BrowserInstaller::new(
        "Karabiner: Vim Mode Plus",
        "A complex modification for Karabiner Elements that mimics Vim's navigation throughout your entire Mac.",
        "karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/vim.json",
    ).install();
    let _ = BrowserInstaller::new(
        "Karabiner: Chinese English",
        "A switch of Chinese & English",
        "karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/chinese-input.json",
    ).install();

    let _ = match BrewFormulaInstaller::new("amethyst", "Till window manager") {
        Ok(installer) => installer.install(),
        Err(error) => return Err(error.into()),
    };

    let _ = AppleStoreInstaller::new(
        "vimlike",
        "Safari extension for vimlike operate experience",
        "vimlike/id1584519802",
    )
    .install();

    Ok(())
}
