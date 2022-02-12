use juniper::FieldResult;
use serde::Serialize;

use crate::models::category::ResCategories;

pub struct QueryRoot;

#[derive(GraphQLObject, Clone, Serialize)]
struct Human {
    uuid: String,
    name: String,
}

/// GET 系
#[juniper::graphql_object]
impl QueryRoot {
    fn categories() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }
}
