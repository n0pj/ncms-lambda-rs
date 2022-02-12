use crate::models::user::User;
use diesel::prelude::*;
use ncms_core::{db::mysql::establish_connection, Password};
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct NewLogin {
    pub email: String,
    pub password: String,
}

impl Default for NewLogin {
    fn default() -> Self {
        Self {
            email: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

impl NewLogin {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }

    /// ログインの検証
    pub fn verify_login(&self) -> Result<User, Error> {
        use crate::schema::user::dsl as dsl_user;

        let conn = establish_connection();
        let user = dsl_user::user
            .filter(dsl_user::email.eq(&self.email))
            .first::<User>(&conn);

        // 条件に合う user がいたか、データベースのエラーがあった場合はエラーを返す
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err(Error::new(ErrorKind::Other, "Unkown user")),
        };

        if let None = &user.password {
            return Err(Error::new(ErrorKind::Other, "User has not set password"));
        }

        // パスワードの検証
        let result =
            Password::verify_password(&self.password, user.password.as_ref().unwrap().as_str());

        // verify_password で何らかのエラーがあった場合
        let result = match result {
            Ok(result) => result,
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
        };

        if result == true {
            Ok(user)
        } else {
            Err(Error::new(ErrorKind::Other, "failed to verify password"))
        }
    }
}
