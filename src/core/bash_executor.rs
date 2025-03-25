use std::process::{Command, Stdio};

use crate::traits::Executor;

pub struct BashExecutor {
    path: String,
}

impl BashExecutor {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Executor for BashExecutor {
    fn execute(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = Command::new("bash")
            .arg(self.path.to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(status.success())
    }
}
