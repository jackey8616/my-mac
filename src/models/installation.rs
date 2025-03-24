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
