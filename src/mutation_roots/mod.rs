use juniper::FieldResult;
// use serde::Serialize;

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn test() -> FieldResult<String> {
        Ok("".to_owned())
    }
}
