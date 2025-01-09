use core::fmt;

use rules::OwnerNameCannotBeEmpty;
use uuid::Uuid;

use super::shared::RuleChecker;
mod rules;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Owner {
    id: Uuid,
    name: OwnerName,
}

impl Owner {
    pub fn new(id: Uuid, name: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            name: OwnerName::new(name)?,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &OwnerName {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerName(String);

impl RuleChecker for OwnerName {}

impl OwnerName {
    pub fn new(raw: &str) -> Result<Self, anyhow::Error> {
        let trimmed = raw.trim();
        Self::check_rule(OwnerNameCannotBeEmpty::new(trimmed))?;
        Ok(Self(trimmed.to_string()))
    }
}

impl fmt::Display for OwnerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
