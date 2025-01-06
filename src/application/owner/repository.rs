use crate::domain::Owner;

pub trait OwnerRepository {
    fn create_owner(&self, req: &CreateOwnerRequest) -> Result<Owner, anyhow::Error>;
}
