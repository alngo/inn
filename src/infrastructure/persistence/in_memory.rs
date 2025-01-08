use std::sync::RwLock;

use async_trait::async_trait;

use crate::{
    application::{create_owner, OwnerRepository},
    domain::Owner,
};

#[derive(Default, Debug)]
pub struct InMemory {
    owners: RwLock<Vec<Owner>>,
}

#[async_trait(?Send)]
impl OwnerRepository for InMemory {
    async fn create_owner(&self, req: &create_owner::Request) -> Result<Owner, anyhow::Error> {
        let id = uuid::Uuid::now_v7();
        let owner = Owner::new(id, &req.name)?;
        self.owners.write().unwrap().push(owner.clone());
        Ok(owner)
    }
}
