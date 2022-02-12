use crate::schema::post_category;
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
pub struct PostCategory {
    pub uuid: String,
    pub created_at: String,
    pub updated_at: String,
    pub post_uuid: String,
    pub category_uuid: String,
}

impl Model<ResPostCategory, Error> for PostCategory {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResPostCategory, Error> {
        Ok(ResPostCategory {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            category_uuid: self.category_uuid.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "post_category"]
pub struct NewPostCategory {
    pub uuid: String,
    pub post_uuid: String,
    pub category_uuid: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for NewPostCategory {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            post_uuid: "".to_owned(),
            category_uuid: "".to_owned(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<PostCategory, NewPostCategory, Error> for NewPostCategory {
    fn from_model(model: &PostCategory) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            post_uuid: model.post_uuid.clone(),
            category_uuid: model.category_uuid.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<PostCategory, Error> {
        Ok(PostCategory {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            category_uuid: self.category_uuid.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn insert(&self) -> Result<PostCategory, Error> {
        Ok(PostCategory {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            category_uuid: self.category_uuid.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn update(&self) -> Result<PostCategory, Error> {
        Ok(PostCategory {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            category_uuid: self.category_uuid.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
pub struct ResPostCategory {
    pub uuid: String,
    pub post_uuid: String,
    pub category_uuid: String,
    pub created_at: String,
    pub updated_at: String,
}

pub type PostCategories = Vec<PostCategory>;
pub type ResPostCategories = Vec<ResPostCategory>;

pub trait PostCategoriesMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResPostCategories;
}

impl PostCategoriesMethods for PostCategories {
    fn to_res(&self) -> ResPostCategories {
        self.into_iter()
            .map(|post| post.to_res().unwrap())
            .collect()
    }
}
