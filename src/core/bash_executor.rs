use std::process::{Command, Stdio};

use crate::traits::ScriptExecutor;

pub struct BashExecutor;

impl ScriptExecutor for BashExecutor {
    fn execute(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let status = Command::new("bash")
            .arg(path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(status.success())
    }
}
