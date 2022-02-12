use crate::models::post_comment::{PostComment, PostCommentsMethods, ResPostComments};
use diesel::prelude::*;
use juniper::FieldResult;
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgPostComment {
    pub uuid: String,
}

pub fn post_comments(arg_post_comment: Option<ArgPostComment>) -> FieldResult<ResPostComments> {
    use crate::schema::post_comment::dsl as dsl_post_comment;

    let conn = establish_connection();
    let post_comments = dsl_post_comment::post_comment.load::<PostComment>(&conn)?;

    println!("{:?}", post_comments);

    Ok(post_comments.to_res())
}
