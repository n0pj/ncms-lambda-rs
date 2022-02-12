use crate::models::category::{CategoriesMethods, Category, ResCategories};
use diesel::prelude::*;
use juniper::FieldResult;
use ncms_core::db::mysql::establish_connection;

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ArgCategory {
    pub uuid: String,
}

pub fn categories(arg_category: Option<ArgCategory>) -> FieldResult<ResCategories> {
    use crate::schema::category::dsl as dsl_category;

    let conn = establish_connection();
    let categories = dsl_category::category.load::<Category>(&conn)?;

    println!("{:?}", categories);

    Ok(categories.to_res())
}
