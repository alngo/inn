#[cfg(test)]
use mockall::automock;

use super::error::MsgError;

#[cfg_attr(test, automock)]
pub trait Rule {
    fn is_valid(&self) -> bool;
    fn message(&self) -> String;
}

pub trait RuleChecker {
    fn check_rule(rule: impl Rule) -> Result<(), MsgError> {
        if !rule.is_valid() {
            return Err(MsgError {
                message: rule.message(),
            });
        }
        Ok(())
    }
}
