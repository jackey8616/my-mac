#[derive(Clone)]
pub enum InstallationStepAction {
    BrowserOpen(String, bool),
    InternetScriptInstall(String, String),
    BrewFormulaInstall(String),
    AppleStoreOpen(String),
}
