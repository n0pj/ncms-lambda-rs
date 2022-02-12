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

    fn post_categories() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn posts() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn post() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn post_comments() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn users() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn login() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

    fn logout() -> FieldResult<Vec<Human>> {
        Ok(vec![Human {
            uuid: "1".to_string(),
            name: "category1".to_string(),
        }])
    }

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
