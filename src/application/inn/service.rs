use async_trait::async_trait;

use super::owner::create_owner;

#[async_trait(?Send)]
pub trait Service {
    async fn create_owner(&self, name: &str) -> create_owner::Result;
}
