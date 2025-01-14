use crate::{application::inn::owner::create_owner, interface::shared::Present};

pub mod cli;

pub trait OwnerPresenter: Present<create_owner::Result> {}
