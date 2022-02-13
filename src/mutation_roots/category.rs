use crate::models::category::{Category, Model, NewCategory, NewModel, ResCategory};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject};
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgCreateCategory {
    pub name: String,
    pub slug: String,
    pub is_main: bool,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgUpdateCategory {
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub is_main: bool,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgDeleteCategory {
    pub uuid: String,
}

pub fn create_category(arg_category: ArgCreateCategory) -> FieldResult<ResCategory> {
    use crate::schema::category::dsl as dsl_category;

    let conn = establish_connection();
    let new_category = NewCategory {
        name: arg_category.name,
        slug: arg_category.slug,
        is_main: arg_category.is_main,
        ..Default::default()
    };
    let _result = diesel::insert_into(dsl_category::category)
        .values(&new_category)
        .execute(&conn)?;

    Ok(new_category.to_model()?.to_res()?)
}

pub fn update_category(arg_category: ArgUpdateCategory) -> FieldResult<ResCategory> {
    use crate::schema::category::dsl as dsl_category;

    let conn = establish_connection();
    let new_category = NewCategory {
        uuid: arg_category.uuid,
        name: arg_category.name,
        slug: arg_category.slug,
        is_main: arg_category.is_main,
        ..Default::default()
    };
    let _result = diesel::update(dsl_category::category.find(&new_category.uuid))
        .set((
            dsl_category::name.eq(&new_category.name),
            dsl_category::slug.eq(&new_category.slug),
            dsl_category::is_main.eq(&new_category.is_main),
        ))
        .execute(&conn)?;

    Ok(new_category.to_model()?.to_res()?)
}

pub fn delete_category(arg_category: ArgDeleteCategory) -> FieldResult<ResCategory> {
    use crate::schema::category::dsl as dsl_category;

    let conn = establish_connection();
    let category = dsl_category::category
        .find(&arg_category.uuid)
        .first::<Category>(&conn)?;
    let _result = diesel::delete(dsl_category::category.find(&arg_category.uuid)).execute(&conn)?;

    Ok(category.to_res()?)
}
