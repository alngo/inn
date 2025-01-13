use crate::{application::inn::owner::create_owner, interface::shared::Present};

#[derive(Default)]
pub struct Presenter;

impl Present<create_owner::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: create_owner::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Owner created: {}", data.owner.name()),
            Err(err) => format!("Unable create owner: {}", err),
        }
    }
}
