use crate::domain::Owner;

use super::use_cases::*;

pub trait OwnerRepository {
    fn create_owner(&self, req: &create_owner::Request) -> Result<Owner, create_owner::Error>;
}
