use crate::models::post::{Model, NewModel, NewPost, Post, ResPost};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject};
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgCreatePost {
    pub status_uuid: String,
    pub user_uuid: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub content: String,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgUpdatePost {
    pub uuid: String,
    pub status_uuid: String,
    pub user_uuid: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub content: String,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgDeletePost {
    pub uuid: String,
}

pub fn create_post(arg_post: ArgCreatePost) -> FieldResult<ResPost> {
    use crate::schema::post::dsl as dsl_post;

    let conn = establish_connection();
    let new_post = NewPost {
        status_uuid: arg_post.status_uuid,
        user_uuid: arg_post.user_uuid,
        title: arg_post.title,
        password: arg_post.password,
        content: arg_post.content,
        slug: arg_post.slug,
        ..Default::default()
    };
    let _result = diesel::insert_into(dsl_post::post)
        .values(&new_post)
        .execute(&conn)?;

    Ok(new_post.to_model()?.to_res()?)
}

pub fn update_post(arg_post: ArgUpdatePost) -> FieldResult<ResPost> {
    use crate::schema::post::dsl as dsl_post;

    let conn = establish_connection();
    let new_post = NewPost {
        uuid: arg_post.uuid,
        status_uuid: arg_post.status_uuid,
        user_uuid: arg_post.user_uuid,
        title: arg_post.title,
        password: arg_post.password,
        content: arg_post.content,
        slug: arg_post.slug,
        ..Default::default()
    };
    let _result = diesel::update(dsl_post::post.find(&new_post.uuid))
        .set((
            dsl_post::status_uuid.eq(&new_post.status_uuid),
            dsl_post::user_uuid.eq(&new_post.user_uuid),
            dsl_post::title.eq(&new_post.title),
            dsl_post::password.eq(&new_post.password),
            dsl_post::content.eq(&new_post.content),
            dsl_post::slug.eq(&new_post.slug),
        ))
        .execute(&conn)?;

    Ok(new_post.to_model()?.to_res()?)
}

pub fn delete_post(arg_post: ArgDeletePost) -> FieldResult<ResPost> {
    use crate::schema::post::dsl as dsl_post;

    let conn = establish_connection();
    let post = dsl_post::post.find(&arg_post.uuid).first::<Post>(&conn)?;
    let _result = diesel::delete(dsl_post::post.find(&arg_post.uuid)).execute(&conn)?;

    Ok(post.to_res()?)
}
