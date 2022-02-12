use crate::schema::category;
use chrono::Utc;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use ncms_core::{Model, NewModel};
use std::io::Error;
use uuid::Uuid;

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

impl Model<ResCategory, Error> for Category {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResCategory, Error> {
        Ok(ResCategory {
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
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "category"]
pub struct NewCategory {
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub is_main: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for NewCategory {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            name: "".to_owned(),
            slug: "".to_owned(),
            is_main: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<Category, NewCategory, Error> for NewCategory {
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
pub struct ResCategory {
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub is_main: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub type Categories = Vec<Category>;
pub type ResCategories = Vec<ResCategory>;

pub trait CategoriesMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResCategories;
}

impl CategoriesMethods for Categories {
    fn to_res(&self) -> ResCategories {
        self.into_iter()
            .map(|post| post.to_res().unwrap())
            .collect()
    }
}
