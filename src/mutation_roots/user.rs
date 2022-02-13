use crate::models::user::{Model, NewModel, NewUser, ResUser, User};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject};
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgCreateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub google_authenticator_secret: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgUpdateUser {
    pub uuid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub google_authenticator_secret: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgDeleteUser {
    pub uuid: String,
}

pub fn create_user(arg_user: ArgCreateUser) -> FieldResult<ResUser> {
    use crate::schema::user::dsl as dsl_user;

    let conn = establish_connection();
    let new_user = NewUser {
        name: arg_user.name,
        email: arg_user.email,
        display_name: arg_user.display_name,
        password: arg_user.password,
        google_authenticator_secret: arg_user.google_authenticator_secret,
        ..Default::default()
    };
    let _result = diesel::insert_into(dsl_user::user)
        .values(&new_user)
        .execute(&conn)?;

    Ok(new_user.to_model()?.to_res()?)
}

pub fn update_user(arg_user: ArgUpdateUser) -> FieldResult<ResUser> {
    use crate::schema::user::dsl as dsl_user;

    let conn = establish_connection();
    let new_user = NewUser {
        uuid: arg_user.uuid,
        name: arg_user.name,
        email: arg_user.email,
        display_name: arg_user.display_name,
        password: arg_user.password,
        google_authenticator_secret: arg_user.google_authenticator_secret,
        ..Default::default()
    };
    let _result = diesel::update(dsl_user::user.find(&new_user.uuid))
        .set((
            dsl_user::name.eq(&new_user.name),
            dsl_user::email.eq(&new_user.email),
            dsl_user::display_name.eq(&new_user.display_name),
            dsl_user::password.eq(&new_user.password),
            dsl_user::google_authenticator_secret.eq(&new_user.google_authenticator_secret),
        ))
        .execute(&conn)?;

    Ok(new_user.to_model()?.to_res()?)
}

pub fn delete_user(arg_user: ArgDeleteUser) -> FieldResult<ResUser> {
    use crate::schema::user::dsl as dsl_user;

    let conn = establish_connection();
    let user = dsl_user::user.find(&arg_user.uuid).first::<User>(&conn)?;
    let _result = diesel::delete(dsl_user::user.find(&arg_user.uuid)).execute(&conn)?;

    Ok(user.to_res()?)
}
