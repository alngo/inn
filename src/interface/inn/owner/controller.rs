use std::sync::Arc;

use crate::{
    application::{
        inn::owner::{create_owner, OwnerRepository},
        shared::UseCase,
    },
    interface::shared::Present,
};

#[derive(Debug)]
pub struct Controller<R, P> {
    repository: Arc<R>,
    presenter: P,
}

impl<R, P> Clone for Controller<R, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        let repository = Arc::clone(&self.repository);
        let presenter = self.presenter.clone();
        Self {
            repository,
            presenter,
        }
    }
}

impl<R, P> Controller<R, P>
where
    R: OwnerRepository,
    P: Present<create_owner::Result>,
{
    pub const fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn create_owner(
        &self,
        req: &create_owner::Request,
    ) -> <P as Present<create_owner::Result>>::ViewModel {
        let use_case = create_owner::CreateOwner::new(&*self.repository);
        let res = use_case.execute(req).await;
        self.presenter.present(res)
    }
}
