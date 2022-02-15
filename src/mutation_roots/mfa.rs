use crate::constants::google_authenticator::TITLE;
use crate::models::login::NewLogin;
use crate::models::mfa::ResMfa;
use crate::models::session::{NewSession, ResSession};
use crate::models::user::{Model, NewModel, NewUser};
use crate::mutation_roots::login::ArgVerifyLogin;
use chrono::Utc;
use juniper::{FieldError, FieldResult};
use ncms_core::{gen_jwt_token, Claims, ErrorCorrectionLevel, GoogleAuthenticator};
use serde::Serialize;
use std::env;

// #[derive(Debug, Clone, GraphQLInputObject, Serialize)]
// pub struct ArgCreateMfaSecret {
//     arg_verify_login: ArgVerifyLogin,
//     password: String,
// }

#[derive(Debug, Clone, GraphQLInputObject, Serialize)]
pub struct ArgVerifyMfa {
    arg_verify_login: ArgVerifyLogin,
    code: String,
}

/// ログインの検証をしたあとに、MFA を発行する
pub fn create_mfa_secret(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResMfa> {
    let new_login = NewLogin::new(&arg_verify_login.email, &arg_verify_login.password);

    // ログインの検証
    let user = match new_login.verify_login() {
        Ok(user) => user,
        Err(_) => return Err(FieldError::from("failed to verify login")),
    };

    // MFA を発行
    let mut new_user = NewUser::from_model(&user)?;

    // すでに MFA が発行されている場合はエラー
    if new_user.google_authenticator_secret.is_some() {
        return Err(FieldError::from("mfa secret already exists"));
    }

    let auth = GoogleAuthenticator::new();
    let secret = auth.create_secret(32);
    let qr_code_url = auth.qr_code_url(
        &secret,
        &format!("{}", &user.email.unwrap()),
        TITLE,
        0,
        0,
        ErrorCorrectionLevel::Medium,
    );

    new_user.google_authenticator_secret = Some(secret);

    let user = new_user.update()?;
    let new_mfa = ResMfa {
        user: user.to_res()?,
        qr_code_url,
        ..Default::default()
    };

    Ok(new_mfa)
}

pub fn verify_mfa(arg_mfa: ArgVerifyMfa) -> FieldResult<ResSession> {
    let new_login = NewLogin::new(
        &arg_mfa.arg_verify_login.email,
        &arg_mfa.arg_verify_login.password,
    );
    let user = new_login.verify_login()?;
    let auth = GoogleAuthenticator::new();
    let secret = user.google_authenticator_secret.clone().unwrap();

    // MFA を検証
    // code は Google Authenticator で生成された 6 桁の数字
    let input_code = arg_mfa.code.clone();
    // let code = auth.get_code(&secret, 0)?;
    let is_valid = auth.verify_code(&secret, &input_code, 1, 0);

    // MFA が有効でない場合はエラー
    if !is_valid {
        return Err(FieldError::from("invalid mfa code"));
    }

    // ベアラートークンを発行する
    let now = Utc::now();
    let expired_at = now.clone() + chrono::Duration::days(30);
    let mut new_session = NewSession {
        user_uuid: user.uuid.clone(),
        expired_at: expired_at.to_string(),
        ..Default::default()
    };
    let exp = (now.clone() + chrono::Duration::days(30)).timestamp();
    let iat = now.timestamp();
    let claims = Claims {
        user_uuid: user.uuid.clone(),
        iat,
        exp,
    };

    // .env の JWT_SECRET を取得
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(jwt_secret) => jwt_secret,
        Err(e) => {
            return Err(FieldError::from(format!(
                "failed to get JWT_SECRET: {}",
                e.to_string()
            )))
        }
    };

    new_session.bearer_token = gen_jwt_token(&claims, &jwt_secret, None, None);

    Ok(new_session.insert().unwrap().to_res().unwrap())
}
