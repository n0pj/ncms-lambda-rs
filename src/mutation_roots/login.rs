use crate::models::login::NewLogin;
use crate::models::session::{NewModel, NewSession, ResSession};
use crate::models::user::Model;
use chrono::Utc;
use juniper::FieldResult;
use ncms_core::gen_jwt_token;
use serde::Serialize;

#[derive(Debug, Clone, GraphQLInputObject, Serialize)]
pub struct ArgVerifyLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Claims {
    user_uuid: String,
    iat: i64,
    exp: i64,
}

pub fn verify_login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResSession> {
    let new_login = NewLogin::new(&arg_verify_login.email, &arg_verify_login.password);
    let user = new_login.verify_login()?;

    // ベアラートークンを発行する
    let mut new_session = NewSession {
        user_uuid: user.uuid.clone(),
        ..Default::default()
    };
    let now = Utc::now();
    let exp = (now.clone() + chrono::Duration::days(30)).timestamp();
    let iat = now.timestamp();
    let claims = Claims {
        user_uuid: user.uuid.clone(),
        iat,
        exp,
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap();

    new_session.bearer_token = gen_jwt_token(&claims, &jwt_secret, None, None);

    let session = new_session.insert()?;

    Ok(session.to_res()?)
}
