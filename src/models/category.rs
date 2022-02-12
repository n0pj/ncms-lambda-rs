use crate::schema::category;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use ncms_core::NewModel;
use std::io::Error;

///
/// DB からデータを取得するための構造体
///
#[derive(Debug, Clone, Queryable)]
pub struct Category {
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub is_main: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Category {
    ///
    /// レスポンス用に変換
    ///
    pub fn to_res(&self) -> Category {
        Category {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            is_main: self.is_main.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
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
    pub name: String,
    pub slug: String,
    pub is_main: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for NewCategory {
    fn default() -> Self {
        Self {
            uuid: "".to_owned(),
            name: "".to_owned(),
            slug: "".to_owned(),
            is_main: false,
            created_at: "".to_owned(),
            updated_at: "".to_owned(),
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
            name: model.name.clone(),
            slug: model.slug.clone(),
            is_main: model.is_main.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            is_main: self.is_main.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
    fn insert(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            is_main: self.is_main.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
    fn update(&self) -> Result<Category, Error> {
        Ok(Category {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            is_main: self.is_main.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
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
