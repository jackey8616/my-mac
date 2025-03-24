use std::error::Error;

pub trait Installer {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn is_installed(&self) -> Result<bool, Box<dyn Error>>;
    fn install(&self) -> Result<(), Box<dyn Error>>;
}
