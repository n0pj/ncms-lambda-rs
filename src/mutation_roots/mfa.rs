use crate::constants::google_authenticator::TITLE;
use crate::models::login::NewLogin;
use crate::models::mfa::ResMfa;
use crate::models::session::{NewSession, ResSession};
use crate::models::user::{Model, NewModel, NewUser};
use crate::mutation_roots::login::ArgVerifyLogin;
use juniper::{FieldError, FieldResult};
use ncms_core::{ErrorCorrectionLevel, GoogleAuthenticator};
use serde::Serialize;

// #[derive(Debug, Clone, GraphQLInputObject, Serialize)]
// pub struct ArgCreateMfaSecret {
//     arg_verify_login: ArgVerifyLogin,
//     password: String,
// }

#[derive(Debug, Clone, GraphQLInputObject, Serialize)]
pub struct ArgConfirmMfa {
    password: String,
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

pub fn confirm_mfa(arg_mfa: ArgConfirmMfa) -> FieldResult<ResSession> {
    let new_session = NewSession {
        ..Default::default()
    };

    Ok(new_session.insert().unwrap().to_res().unwrap())
}
