use rocket::{async_trait, Request};
use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use crate::config::Config;

pub struct ApiKey;

#[async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(config): Option<&Config> = request.rocket().state() else {
            return Outcome::Error((Status::Unauthorized, ()));  
        };
        
        let Some(token) = request.headers().get_one("Authorization") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        
        if config.get_auth_tokens().await.contains(token.to_string()) {
            Outcome::Success(ApiKey)
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
