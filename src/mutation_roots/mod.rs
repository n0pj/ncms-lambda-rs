mod category;
mod login;
mod logout;
mod mfa;
mod post;
mod post_comment;
mod user;

use juniper::FieldResult;
// use serde::Serialize;
use crate::models::category::ResCategory;
use crate::models::mfa::ResMfa;
use crate::models::post::ResPost;
use crate::models::post_comment::ResPostComment;
use crate::models::session::ResSession;
use crate::models::user::ResUser;

use crate::mutation_roots::mfa::{confirm_mfa, create_mfa_secret, ArgConfirmMfa};
use category::{
    create_category, delete_category, update_category, ArgCreateCategory, ArgDeleteCategory,
    ArgUpdateCategory,
};
use login::{verify_login, ArgVerifyLogin};
use post::{create_post, delete_post, update_post, ArgCreatePost, ArgDeletePost, ArgUpdatePost};
use post_comment::{
    create_post_comment, delete_post_comment, update_post_comment, ArgCreatePostComment,
    ArgDeletePostComment, ArgUpdatePostComment,
};
use user::{create_user, delete_user, update_user, ArgCreateUser, ArgDeleteUser, ArgUpdateUser};

use ncms_core::Header;

pub struct MutationRoot {
    pub header: Header,
}

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

    fn create_post(arg_post: ArgCreatePost) -> FieldResult<ResPost> {
        create_post(arg_post)
    }

    fn update_post(arg_post: ArgUpdatePost) -> FieldResult<ResPost> {
        update_post(arg_post)
    }

    fn delete_post(arg_post: ArgDeletePost) -> FieldResult<ResPost> {
        delete_post(arg_post)
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

    fn create_user(arg_user: ArgCreateUser) -> FieldResult<ResUser> {
        create_user(arg_user)
    }

    fn update_user(arg_user: ArgUpdateUser) -> FieldResult<ResUser> {
        update_user(arg_user)
    }

    fn delete_user(arg_user: ArgDeleteUser) -> FieldResult<ResUser> {
        delete_user(arg_user)
    }

    // fn verify_login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResUser> {
    //     verify_login(arg_verify_login)
    // }

    fn login(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResSession> {
        verify_login(arg_verify_login)
    }

    // fn logout() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

    /// MFA 作成を行う。
    fn create_mfa_secret(arg_verify_login: ArgVerifyLogin) -> FieldResult<ResMfa> {
        create_mfa_secret(arg_verify_login)
    }

    /// 2 要素認証確認用
    fn confirm_mfa(arg_mfa: ArgConfirmMfa) -> FieldResult<ResSession> {
        confirm_mfa(arg_mfa)
    }
}
