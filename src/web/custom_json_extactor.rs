use serde::Deserialize;
use axum::{async_trait, extract::FromRequest, Json};
use axum::http::{Request, StatusCode};
use axum::response::{Response, IntoResponse};
use axum::body::{Body, Bytes};
use validator::{Validate, ValidationErrors};


#[derive(Deserialize, Validate, Debug)]
pub struct RequestUser {
  #[validate(email(message="must be a valid email id"))]
   pub username: String,
   #[validate(length(min=8, message="must be at least 8 charactors"))]
   pub  password: String,
}

fn do_thing_with_request_body(bytes: Bytes) {
  tracing::debug!(body = ?bytes);
  //let user: RequestUser = body.
}

pub struct ValidatedBody(Bytes);

#[async_trait]
impl<S> FromRequest<S> for ValidatedBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // do validation...
        do_thing_with_request_body(body.clone());

        Ok(Self(body))
    }
}


#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    Body: FromRequest<S>,
    S: Send + Sync,
{
    //type Rejection = Response;
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user):Json<RequestUser> = Json::from_request(req, state)
            .await
           //.map_err(IntoResponse::into_response)?;
           .map_err(|er| (StatusCode::BAD_REQUEST, format!("{}", er)))?;

        // do validation...
        // //do_thing_with_request_body(body.clone());
        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", err)));
        }

        Ok(user)
    }
}




// pub async fn custom_json_extactor(ValidatedBody(body):ValidatedBody) {
//   tracing::debug!(?body, "handler received body");
  
// }

pub async fn custom_json_extactor(user:RequestUser) {
  tracing::debug!(?user, "handler received body");
  
}