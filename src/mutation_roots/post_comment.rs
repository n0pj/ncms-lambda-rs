use crate::models::post_comment::{Model, NewModel, NewPostComment, PostComment, ResPostComment};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject};
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgCreatePostComment {
    pub content: String,
    pub post_uuid: String,
    pub user_uuid: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgUpdatePostComment {
    pub uuid: String,
    pub content: String,
    pub post_uuid: String,
    pub user_uuid: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgDeletePostComment {
    pub uuid: String,
}

pub fn create_post_comment(arg_post_comment: ArgCreatePostComment) -> FieldResult<ResPostComment> {
    use crate::schema::post_comment::dsl as dsl_post_comment;

    let conn = establish_connection();
    let new_post_comment = NewPostComment {
        post_uuid: arg_post_comment.post_uuid,
        user_uuid: arg_post_comment.user_uuid,
        content: arg_post_comment.content,
        ..Default::default()
    };
    let _result = diesel::insert_into(dsl_post_comment::post_comment)
        .values(&new_post_comment)
        .execute(&conn)?;

    Ok(new_post_comment.to_model()?.to_res()?)
}

pub fn update_post_comment(arg_post_comment: ArgUpdatePostComment) -> FieldResult<ResPostComment> {
    use crate::schema::post_comment::dsl as dsl_post_comment;

    let conn = establish_connection();
    let new_post_comment = NewPostComment {
        uuid: arg_post_comment.uuid,
        post_uuid: arg_post_comment.post_uuid,
        user_uuid: arg_post_comment.user_uuid,
        content: arg_post_comment.content,
        ..Default::default()
    };
    let _result = diesel::update(dsl_post_comment::post_comment.find(&new_post_comment.uuid))
        .set((
            dsl_post_comment::post_uuid.eq(&new_post_comment.post_uuid),
            dsl_post_comment::user_uuid.eq(&new_post_comment.user_uuid),
            dsl_post_comment::content.eq(&new_post_comment.content),
        ))
        .execute(&conn)?;

    Ok(new_post_comment.to_model()?.to_res()?)
}

pub fn delete_post_comment(arg_post_comment: ArgDeletePostComment) -> FieldResult<ResPostComment> {
    use crate::schema::post_comment::dsl as dsl_post_comment;

    let conn = establish_connection();
    let post_comment = dsl_post_comment::post_comment
        .find(&arg_post_comment.uuid)
        .first::<PostComment>(&conn)?;
    let _result = diesel::delete(dsl_post_comment::post_comment.find(&arg_post_comment.uuid))
        .execute(&conn)?;

    Ok(post_comment.to_res()?)
}
