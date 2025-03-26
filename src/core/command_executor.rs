use std::{error::Error, process::Command};

use crate::traits::Executor;

pub struct CommandExecutor {
    cmd: String,
    args: Vec<String>,
}

impl CommandExecutor {
    pub fn new(cmd: String, args: Vec<String>) -> Self {
        Self { cmd, args }
    }
}

impl Executor<String> for CommandExecutor {
    fn execute(&self) -> Result<String, Box<dyn Error>> {
        let args: Vec<&str> = self.args.iter().map(|arg| arg.as_str()).collect();
        let output = Command::new(self.cmd.as_str()).args(args).output()?;

        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    }
}

#[cfg(test)]
mod command_executor_tests {
    use super::*;

    #[test]
    fn test_initializer() {
        let executor = CommandExecutor::new("bash".to_string(), vec!["some_link".to_string()]);

        assert_eq!(executor.cmd, "bash");
        assert_eq!(executor.args, vec!["some_link"]);
    }

    #[test]
    fn test_execute() {
        let executor = CommandExecutor::new("bash".to_string(), vec!["--help".to_string()]);

        let result = executor.execute();

        assert_eq!(result.is_ok(), true);
    }
}
