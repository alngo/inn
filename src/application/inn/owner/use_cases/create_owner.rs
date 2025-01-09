use async_trait::async_trait;

use crate::{
    application::{
        inn::owner::{rules::OwnerNameMustBeUnique, OwnerRepository}, shared::UseCase
    },
    domain::{Owner, RuleChecker},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Request {
    pub name: String,
}

pub struct Response {
    pub owner: Owner,
}

pub type Result = std::result::Result<Response, anyhow::Error>;

pub struct CreateOwner<'a, A> {
    owner_repository: &'a A,
}

impl<'a, A> CreateOwner<'a, A>
where
    A: OwnerRepository + std::marker::Sync,
{
    pub fn new(owner_repository: &'a A) -> Self {
        Self { owner_repository }
    }
}

impl<'a, A> RuleChecker for CreateOwner<'a, A> where A: OwnerRepository {}

#[async_trait(?Send)]
impl<'a, A> UseCase for CreateOwner<'a, A>
where
    A: OwnerRepository + std::marker::Sync,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: &Self::Request) -> Result {
        let existing_names = vec![]; // TODO: Fetch existing names from repository
        Self::check_rule(OwnerNameMustBeUnique::new(&request.name, existing_names))?;
        let owner = self.owner_repository.create_owner(&request).await?;
        Ok(Response { owner })
    }
}

#[cfg(test)]
mod use_case_create_owner_tests {
    use crate::{application::inn::owner::repository::MockOwnerRepository, domain::OwnerName};

    use super::*;

    #[tokio::test]
    async fn test_create_owner() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_create_owner()
            .return_once(|req| {
                let owner = Owner::new(uuid::Uuid::now_v7(), &req.name).unwrap();
                Ok(owner)
            });
        let create_owner = CreateOwner::new(&owner_repository);
        let request = Request {
            name: "Alice".to_string(),
        };
        let response = create_owner.execute(&request).await.unwrap();
        assert_eq!(response.owner.name(), &OwnerName::new("Alice").unwrap());
    }
}
