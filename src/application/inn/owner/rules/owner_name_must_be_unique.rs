use thiserror::Error;

use crate::domain::Rule;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("owner name {name} must be unique")]
pub struct OwnerNameMustBeUnique {
    name: String,
    existing_names: Vec<String>,
}

impl OwnerNameMustBeUnique {
    pub fn new(name: &str, existing_names: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            existing_names,
        }
    }
}

impl Rule for OwnerNameMustBeUnique {
    fn is_valid(&self) -> bool {
        !self.existing_names.contains(&self.name)
    }

    fn error(&self) -> anyhow::Error {
        self.clone().into()
    }
}

#[cfg(test)]
mod owner_name_must_be_unique_tests {
    use super::*;

    #[test]
    fn test_owner_name_must_be_unique_error() {
        let rule = OwnerNameMustBeUnique::new("John Doe", vec!["John Doe".to_string()]);
        assert!(!rule.is_valid());
        assert_eq!(
            rule.error().to_string(),
            "owner name John Doe must be unique"
        );
    }

    #[test]
    fn test_owner_name_must_be_unique() {
        let rule = OwnerNameMustBeUnique::new("John Doe", vec!["Jane Doe".to_string()]);
        assert!(rule.is_valid());
    }
}
