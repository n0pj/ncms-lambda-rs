use crate::models::user::User;
use ncms_core::{verify_jwt_token, Claims};
use ncms_core::{Authorization, AuthorizationMethods};
use std::io::{Error, ErrorKind};

// Bearer Token を検証する
pub fn verify_bearer_token(authorization: Authorization) -> Result<User, Error> {
    // search session from bearer token
    use diesel::prelude::*;
    use ncms_core::db::mysql::establish_connection;

    let token = match authorization {
        Some(token) => {
            Some(token).get_token()?
            // create_kindergarten(arg_kindergarten, uuid)
        }
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Bearer Token is not found.",
            ))
        }
    };

    let conn = establish_connection();

    // .env の JWT_SECRET を取得
    let jwt_secret = dotenv::var("JWT_SECRET").unwrap();

    // Bearer Token が有効であるかを検証する
    let token_data = match verify_jwt_token::<Claims>(&token, &jwt_secret) {
        Ok(token_data) => {
            // 検証に成功した場合は、Bearer Token から Session を取得する
            token_data
        }
        Err(_) => return Err(Error::new(ErrorKind::Other, "Invalid Bearer Token")),
    };

    // println!("{:?}", token_data);

    // search user from session.user_uuid
    // ユーザーが存在するかを検証する
    use crate::schema::user::dsl as dsl_user;

    let user = dsl_user::user
        .find(&token_data.claims.user_uuid)
        .first::<User>(&conn);

    match user {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::new(ErrorKind::Other, "Invalid Bearer Token")),
    }
}
