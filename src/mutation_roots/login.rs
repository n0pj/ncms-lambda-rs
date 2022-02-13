use crate::models::login::NewLogin;
use crate::models::session::{NewModel, NewSession, ResSession};
use crate::models::user::Model;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use juniper::FieldResult;
use serde::Serialize;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgVerifyLogin {
    pub email: String,
    pub password: String,
}

pub fn verify_login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResSession> {
    let new_login = NewLogin::new(&arg_verify_login.email, &arg_verify_login.password);
    let user = new_login.verify_login()?;

    // ベアラートークンを発行する
    let mut session = NewSession {
        user_uuid: user.uuid.clone(),
        ..Default::default()
    };

    session.bearer_token = gen_jwt_token(&session.uuid, &session.token_secret);

    let result = session.insert()?;

    Ok(result.to_res()?)
}

#[derive(Debug, Clone, Serialize)]
pub struct Claims<'a> {
    user_id: &'a str,
    iat: i64,
    exp: i64,
}

fn gen_jwt_token(user_id: &str, secret: &str) -> String {
    let mut header = Header::new(Algorithm::HS256);
    let now = Utc::now();
    let iat = now.clone().timestamp();
    let exp = (now + chrono::Duration::days(30)).timestamp();
    let claims = Claims { user_id, iat, exp };

    header.typ = Some("JWT".to_string());

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Unable to generate token")
}
