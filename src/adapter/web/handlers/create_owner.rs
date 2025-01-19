use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    adapter::{
        inn::owner,
        shared::Present,
        web::presenter::{ApiError, ApiSuccess, Presenter},
    },
    application::inn::owner::{create_owner, OwnerRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateOwnerHttpRequestBody {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateOwnerResponseData {
    id: String,
}

type WebResult = Result<ApiSuccess<CreateOwnerResponseData>, ApiError>;

impl Present<create_owner::Result> for Presenter {
    type ViewModel = WebResult;

    fn present(&self, result: create_owner::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Owner created: {}", data.owner.name()),
            Err(err) => format!("Unable create owner: {}", err),
        }
    }
}

pub async fn create_owner<R, P>(
    State(controller): State<owner::Controller<R, P>>,
    Json(body): Json<CreateOwnerHttpRequestBody>,
) -> WebResult
where
    R: OwnerRepository + Sync + Send + 'static,
    P: Present<create_owner::Result> + Clone + Send + Sync + 'static,
{
    let request = create_owner::Request {
        name: body.name.clone(),
    };
    controller.create_owner(&request).await
}
