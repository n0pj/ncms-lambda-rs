use crate::models::login::NewLogin;
use crate::models::user::{Model, ResUser};
use juniper::FieldResult;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgVerifyLogin {
    pub email: String,
    pub password: String,
}

pub fn verify_login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResUser> {
    let new_login = NewLogin::new(&arg_verify_login.email, &arg_verify_login.password);
    let user = new_login.verify_login()?;

    user.to_res()
}
