use super::installation_step_action::InstallationStepAction;

#[derive(Clone)]
pub struct InstallationStep {
    pub name: String,
    pub description: String,
    pub action: InstallationStepAction,
    pub optional: bool,
}

impl InstallationStep {
    pub fn new(name: &str, description: &str, action: InstallationStepAction) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            action,
            optional: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

#[cfg(test)]
mod installation_step_tests {
    use crate::models::{InstallationStep, InstallationStepAction};

    #[test]
    fn test_installation_step_optional() {
        let step = InstallationStep::new(
            "Optional Step",
            "A test optional step",
            InstallationStepAction::BrowserOpen("https://example.com".to_string(), false),
        )
        .optional();

        assert!(step.optional);
        assert_eq!(step.name, "Optional Step");
    }
}
