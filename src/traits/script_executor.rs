use std::error::Error;

pub trait ScriptExecutor {
    fn execute(&self, path: &str) -> Result<bool, Box<dyn Error>>;
}
