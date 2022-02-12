use crate::models::post::{Model, Post, PostsMethods, ResPost, ResPosts};
use diesel::prelude::*;
use juniper::FieldResult;
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgPost {
    pub uuid: String,
}

pub fn posts(arg_post: Option<ArgPost>) -> FieldResult<ResPosts> {
    use crate::schema::post::dsl as dsl_post;

    let conn = establish_connection();
    let posts = dsl_post::post.load::<Post>(&conn)?;

    println!("{:?}", posts);

    Ok(posts.to_res())
}

pub fn post(arg_post: Option<ArgPost>) -> FieldResult<ResPost> {
    use crate::schema::post::dsl as dsl_post;

    let conn = establish_connection();
    let post = dsl_post::post
        .find(arg_post.unwrap().uuid)
        .first::<Post>(&conn)?;

    println!("{:?}", post);

    Ok(post.to_res()?)
}
