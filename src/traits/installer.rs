use std::error::Error;

pub trait Installer {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn is_installed(&self) -> Result<bool, Box<dyn Error>>;
    fn install(&self) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod installer_tests {
    use crate::traits::Installer;

    #[test]
    fn test_mock_installer() {
        struct MockInstaller {
            is_installed: bool,
        }

        impl Installer for MockInstaller {
            fn name(&self) -> &str {
                "MockTool"
            }
            fn description(&self) -> &str {
                "A mock installer for testing"
            }

            fn is_installed(&self) -> Result<bool, Box<dyn std::error::Error>> {
                Ok(self.is_installed)
            }

            fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
                if !self.is_installed {
                    // Simulate installation
                    Ok(())
                } else {
                    Ok(())
                }
            }
        }

        let installer_not_installed = MockInstaller {
            is_installed: false,
        };
        let installer_installed = MockInstaller { is_installed: true };

        assert!(
            !installer_not_installed.is_installed().unwrap(),
            "Should report not installed"
        );
        assert!(
            installer_installed.is_installed().unwrap(),
            "Should report installed"
        );
    }
}
