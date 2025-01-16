use crate::{
    application::{
        inn::owner::{create_owner, OwnerRepository},
        shared::UseCase,
    },
    interface::shared::Present,
};

#[derive(Debug, Clone)]
pub struct Controller<'r, 'p, R, P> {
    repository: &'r R, // TODO: change to service
    presenter: &'p P,
}

impl<'r, 'p, R, P> Controller<'r, 'p, R, P>
where
    R: OwnerRepository,
    P: Present<create_owner::Result>,
{
    pub const fn new(repository: &'r R, presenter: &'p P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn create_owner(
        &self,
        req: &create_owner::Request,
    ) -> <P as Present<create_owner::Result>>::ViewModel {
        let use_case = create_owner::CreateOwner::new(self.repository);
        let res = use_case.execute(req).await;
        self.presenter.present(res)
    }
}
