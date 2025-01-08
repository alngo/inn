use async_trait::async_trait;

use crate::domain::Owner;

use super::use_cases::*;

#[async_trait(?Send)]
pub trait OwnerRepository: Sync {
    async fn create_owner(&self, req: &create_owner::Request) -> Result<Owner, anyhow::Error>;
}
