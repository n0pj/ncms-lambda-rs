mod category;
mod login;
mod logout;
mod post;
mod post_comment;
mod user;

use juniper::FieldResult;
// use serde::Serialize;
use crate::models::category::ResCategory;
use crate::models::post_comment::ResPostComment;
use crate::models::user::ResUser;
use category::{
    create_category, delete_category, update_category, ArgCreateCategory, ArgDeleteCategory,
    ArgUpdateCategory,
};
use login::{verify_login, ArgVerifyLogin};
use post_comment::{
    create_post_comment, delete_post_comment, update_post_comment, ArgCreatePostComment,
    ArgDeletePostComment, ArgUpdatePostComment,
};

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_category(arg_category: ArgCreateCategory) -> FieldResult<ResCategory> {
        create_category(arg_category)
    }

    fn update_category(arg_category: ArgUpdateCategory) -> FieldResult<ResCategory> {
        update_category(arg_category)
    }

    fn delete_category(arg_category: ArgDeleteCategory) -> FieldResult<ResCategory> {
        delete_category(arg_category)
    }

    fn create_post_comment(arg_post_comment: ArgCreatePostComment) -> FieldResult<ResPostComment> {
        create_post_comment(arg_post_comment)
    }

    fn update_post_comment(arg_post_comment: ArgUpdatePostComment) -> FieldResult<ResPostComment> {
        update_post_comment(arg_post_comment)
    }

    fn delete_post_comment(arg_post_comment: ArgDeletePostComment) -> FieldResult<ResPostComment> {
        delete_post_comment(arg_post_comment)
    }

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
