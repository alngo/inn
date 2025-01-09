use std::sync::RwLock;

use async_trait::async_trait;

use crate::{
    application::inn::owner::{create_owner, OwnerRepository},
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

#[cfg(test)]
mod owner_repository_in_memory_tests {
    use super::*;
    use crate::{application::inn::owner::create_owner, domain::OwnerName};

    #[tokio::test]
    async fn test_create_owner() {
        let owner_repository = InMemory::default();
        let request = create_owner::Request {
            name: "Alice".to_string(),
        };
        let response = owner_repository.create_owner(&request).await.unwrap();
        assert_eq!(response.name(), &OwnerName::new("Alice").unwrap());
    }
}
