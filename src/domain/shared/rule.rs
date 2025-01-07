#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Rule {
    fn is_valid(&self) -> bool;
    fn error(&self) -> anyhow::Error;
}

pub trait RuleChecker {
    fn check_rule(rule: impl Rule) -> Result<(), anyhow::Error> {
        if !rule.is_valid() {
            return Err(rule.error());
        }
        Ok(())
    }
}
