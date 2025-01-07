use async_trait::async_trait;

use crate::{
    application::{
        owner::{repository::OwnerRepository, rules::OwnerNameMustBeUnique},
        shared::UseCase,
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

pub struct CreateOwner<'a, A> {
    owner_repository: &'a A,
}

impl<'a, A> CreateOwner<'a, A>
where
    A: OwnerRepository,
{
    pub fn new(owner_repository: &'a A) -> Self {
        Self { owner_repository }
    }
}

impl<'a, A> RuleChecker for CreateOwner<'a, A> where A: OwnerRepository {}

#[async_trait(?Send)]
impl<'a, A> UseCase for CreateOwner<'a, A>
where
    A: OwnerRepository,
{
    type Request = Request;
    type Response = Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, anyhow::Error> {
        let existing_names = vec![]; // TODO: Fetch existing names from repository
        Self::check_rule(OwnerNameMustBeUnique::new(&request.name, existing_names))?;
        let owner = self.owner_repository.create_owner(&request)?;
        Ok(Response { owner })
    }
}
