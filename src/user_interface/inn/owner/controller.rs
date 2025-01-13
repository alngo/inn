use crate::{
    application::inn::{owner::create_owner, service::InnService},
    user_interface::shared::Present,
};

pub struct Controller<'s, 'p, S, P> {
    service: &'s S,
    presenter: &'p P,
}

impl<'s, 'p, S, P> Controller<'s, 'p, S, P>
where
    S: InnService,
    P: Present<create_owner::Result>,
{
    pub const fn new(service: &'s S, presenter: &'p P) -> Self {
        Self { service, presenter }
    }

    pub async fn create_owner(
        &self,
        name: &str,
    ) -> <P as Present<create_owner::Result>>::ViewModel {
        let result = self
            .service
            .create_owner(&create_owner::Request {
                name: name.to_string(),
            })
            .await;
        self.presenter.present(result)
    }
}
