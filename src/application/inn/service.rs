use async_trait::async_trait;

use crate::{application::shared::UseCase, domain::Owner};

use super::owner::{create_owner, OwnerRepository};

#[async_trait(?Send)]
pub trait InnService {
    async fn create_owner(&self, req: &create_owner::Request) -> Result<create_owner::Response, anyhow::Error>;
}

#[derive(Debug, Clone)]
pub struct Service<R> 
where
    R: OwnerRepository
{
    repository: R,
}

impl<R> Service<R> 
where
    R: OwnerRepository
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait(?Send)]
impl<R> InnService for Service<R> 
where
    R: OwnerRepository
{
    async fn create_owner(&self, req: &create_owner::Request) -> Result<create_owner::Response, anyhow::Error> {
        let use_case = create_owner::CreateOwner::new(&self.repository);
        use_case.execute(&req).await
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;
    use crate::{application::inn::owner::{create_owner, MockOwnerRepository}, domain::OwnerName};

    #[tokio::test]
    async fn test_create_owner() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_create_owner()
            .return_once(|req| {
                let owner = Owner::new(uuid::Uuid::now_v7(), &req.name).unwrap();
                Ok(owner)
            });
        let service = Service::new(owner_repository);
        let request = create_owner::Request {
            name: "Alice".to_string(),
        };
        let response = service.create_owner(&request).await.unwrap();
        assert_eq!(response.owner.name(), &OwnerName::new("Alice").unwrap());
    }
}
