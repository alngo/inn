use thiserror::Error;

use crate::domain::shared::Rule;

#[derive(Clone, Debug, Error)]
#[error("owner name cannot be empty")]
pub struct OwnerNameCannotBeEmpty {
    name: String,
}

impl OwnerNameCannotBeEmpty {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Rule for OwnerNameCannotBeEmpty {
    fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }

    fn error(&self) -> anyhow::Error {
        self.clone().into()
    }
}

#[cfg(test)]
mod owner_name_cannot_be_empty_tests {
    use super::*;

    #[test]
    fn test_owner_name_cannot_be_empty_error() {
        let rule = OwnerNameCannotBeEmpty::new("");
        assert_eq!(rule.is_valid(), false);
        assert_eq!(rule.error().to_string(), "owner name cannot be empty");
    }

    #[test]
    fn test_owner_name_cannot_be_empty() {
        let rule = OwnerNameCannotBeEmpty::new("John Doe");
        assert_eq!(rule.is_valid(), true);
    }
}
