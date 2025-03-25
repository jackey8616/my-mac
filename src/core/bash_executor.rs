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

#[cfg(test)]
mod bash_executor_tests {
    use super::*;

    #[test]
    fn test_initializer() {
        let executor = BashExecutor::new("some_link");

        assert_eq!(executor.path, "some_link");
    }

    #[test]
    fn test_execute() {
        let executor = BashExecutor::new("--help");

        let result = executor.execute();

        assert_eq!(result.is_ok(), true);
    }
}
