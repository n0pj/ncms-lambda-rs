mod login;
mod logout;

use juniper::FieldResult;
// use serde::Serialize;

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn test() -> FieldResult<String> {
        Ok("".to_owned())
    }

    // fn login() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

    // fn logout() -> FieldResult<Vec<Human>> {
    //     Ok(vec![Human {
    //         uuid: "1".to_string(),
    //         name: "category1".to_string(),
    //     }])
    // }

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
