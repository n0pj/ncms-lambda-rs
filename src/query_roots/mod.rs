mod category;
mod login;
mod logout;
mod post;
mod post_comment;

use juniper::FieldResult;

use crate::models::category::ResCategories;
use crate::models::post::{ResPost, ResPosts};
use crate::models::post_comment::ResPostComments;

use category::{categories, ArgCategory};
use post::{post, posts, ArgPost};
use post_comment::{post_comments, ArgPostComment};

pub struct QueryRoot;

/// GET 系
#[juniper::graphql_object]
impl QueryRoot {
    fn categories(arg_category: Option<ArgCategory>) -> FieldResult<ResCategories> {
        categories(arg_category)
    }

    fn posts(arg_post: Option<ArgPost>) -> FieldResult<ResPosts> {
        posts(arg_post)
    }

    fn post(arg_post: Option<ArgPost>) -> FieldResult<ResPost> {
        post(arg_post)
    }

    fn post_comments(arg_post_comment: Option<ArgPostComment>) -> FieldResult<ResPostComments> {
        post_comments(arg_post_comment)
    }

    // fn users() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

    // fn login() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

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
