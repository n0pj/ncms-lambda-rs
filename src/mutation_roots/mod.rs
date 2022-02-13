mod category;
mod login;
mod logout;
mod post;
mod post_comment;
mod user;

use juniper::FieldResult;
// use serde::Serialize;
use crate::models::user::ResUser;
use login::{verify_login, ArgVerifyLogin};

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn verify_login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResUser> {
        verify_login(arg_verify_login)
    }

    // fn logout() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

    // fn create_mfa_secret() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

    // fn confirm_mfa_secret() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }
}
