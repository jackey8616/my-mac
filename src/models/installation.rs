use super::installation_step::InstallationStep;

#[derive(Clone)]
pub struct Installation {
    pub name: String,
    pub description: String,
    pub is_required: bool,
    pub install_steps: Vec<InstallationStep>,
}

impl Installation {
    pub fn new(name: &str, description: &str, is_required: bool) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            is_required,
            install_steps: Vec::new(),
        }
    }

    pub fn with_install_steps(mut self, steps: Vec<InstallationStep>) -> Self {
        self.install_steps = steps;
        self
    }
}

#[cfg(test)]
mod installation_tests {
    use crate::models::{Installation, InstallationStep, InstallationStepAction};

    #[test]
    fn test_installation_creation() {
        let installation = Installation::new("Test Components", "A test installation", true)
            .with_install_steps(vec![InstallationStep::new(
                "Test Step",
                "A sample installation step",
                InstallationStepAction::BrewFormulaInstall("test-formula".to_string()),
            )]);

        assert_eq!(installation.name, "Test Components");
        assert_eq!(installation.description, "A test installation");
        assert!(installation.is_required);
        assert_eq!(installation.install_steps.len(), 1);
    }
}
