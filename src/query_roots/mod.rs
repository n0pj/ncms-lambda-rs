use juniper::FieldResult;
use serde::Serialize;

pub struct QueryRoot;

#[derive(GraphQLObject, Clone, Serialize)]
struct Human {
    uuid: String,
    name: String,
}

/// GET 系
#[juniper::graphql_object]
impl QueryRoot {
    fn human() -> FieldResult<Human> {
        Ok(Human {
            uuid: "test".to_owned(),
            name: "wasabi".to_owned(),
        })
    }

    fn humans(i: i32) -> FieldResult<Human> {
        let humans = vec![
            Human {
                uuid: "test".to_owned(),
                name: "1".to_owned(),
            },
            Human {
                uuid: "test".to_owned(),
                name: "2".to_owned(),
            },
        ];

        Ok(humans[i as usize].clone())
    }
}
