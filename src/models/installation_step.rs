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
