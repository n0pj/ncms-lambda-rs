use crate::schema::category;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use ncms_core::NewModel;
use std::io::Error;

///
/// DB からデータを取得するための構造体
///
#[derive(Debug, Clone, Queryable)]
struct Category {
    pub uuid: String,
}

impl Category {
    ///
    /// レスポンス用に変換
    ///
    pub fn to_res(&self) -> Category {
        Category {
            uuid: self.uuid.clone(),
        }
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "category"]
struct NewCategory {
    pub uuid: String,
}

impl Default for NewCategory {
    fn default() -> Self {
        Self {
            uuid: "".to_owned(),
        }
    }
}

impl NewModel<Category, NewCategory, Error> for NewCategory {
    ///
    /// DB へデータを作成する
    ///
    fn from_model(model: &Category) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
        })
    }

    fn to_model(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: "".to_owned(),
        })
    }
    fn insert(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: "".to_owned(),
        })
    }
    fn update(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: "".to_owned(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
struct ResCategory {
    pub uuid: String,
}

pub type Categories = Vec<Category>;
pub type NewCategories = Vec<NewCategory>;
pub type ResCategories = Vec<ResCategory>;

pub trait NewCategoriesMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res() -> ResCategories;
}
