use std::sync::Arc;


use crate::{
    application::inn::owner::{create_owner, OwnerRepository},
    interface::shared::Present,
};

use super::owner::controller::Controller;

#[derive(Debug)]
pub struct Service<D, P> {
    database: Arc<D>,
    presenter: P,
}

impl<D, P> Clone for Service<D, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        let database = Arc::clone(&self.database);
        let presenter = self.presenter.clone();
        Self {
            database,
            presenter,
        }
    }
}

impl<D, P> Service<D, P>
where
    D: OwnerRepository + Sync,
    P: Present<create_owner::Result>,
{
    pub fn new(database: Arc<D>, presenter: P) -> Self {
        Self {
            database,
            presenter,
        }
    }

    fn owner_controller(&self) -> Controller<D, P> {
        Controller::new(self.database.as_ref(), &self.presenter)
    }

    pub async fn create_owner(
        &self,
        name: &str,
    ) -> <P as Present<create_owner::Result>>::ViewModel {
        let req = create_owner::Request {
            name: String::from(name),
        };
        self.owner_controller().create_owner(&req).await
    }
}
