use crate::schema::post;
use chrono::Utc;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
pub use ncms_core::{Model, NewModel};
use std::io::Error;
use uuid::Uuid;

///
/// DB からデータを取得するための構造体
///
#[derive(Debug, Clone, Queryable)]
pub struct Post {
    pub uuid: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub content: String,
    pub slug: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status_uuid: String,
    pub user_uuid: String,
}

impl Model<ResPost, Error> for Post {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResPost, Error> {
        Ok(ResPost {
            uuid: self.uuid.clone(),
            status_uuid: self.status_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            title: self.title.clone(),
            password: self.password.clone(),
            content: self.content.clone(),
            slug: self.slug.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "post"]
pub struct NewPost {
    pub uuid: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub content: String,
    pub slug: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status_uuid: String,
    pub user_uuid: String,
}

impl Default for NewPost {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            status_uuid: "".to_owned(),
            user_uuid: "".to_owned(),
            title: None,
            password: None,
            content: "".to_owned(),
            slug: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<Post, NewPost, Error> for NewPost {
    fn from_model(model: &Post) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            status_uuid: model.status_uuid.clone(),
            user_uuid: model.user_uuid.clone(),
            title: model.title.clone(),
            password: model.password.clone(),
            content: model.content.clone(),
            slug: model.slug.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<Post, Error> {
        Ok(Post {
            uuid: self.uuid.clone(),
            status_uuid: self.status_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            title: self.title.clone(),
            password: self.password.clone(),
            content: self.content.clone(),
            slug: self.slug.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn insert(&self) -> Result<Post, Error> {
        Ok(Post {
            uuid: self.uuid.clone(),
            status_uuid: self.status_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            title: self.title.clone(),
            password: self.password.clone(),
            content: self.content.clone(),
            slug: self.slug.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn update(&self) -> Result<Post, Error> {
        Ok(Post {
            uuid: self.uuid.clone(),
            status_uuid: self.status_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            title: self.title.clone(),
            password: self.password.clone(),
            content: self.content.clone(),
            slug: self.slug.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
pub struct ResPost {
    pub uuid: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub content: String,
    pub slug: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status_uuid: String,
    pub user_uuid: String,
}

pub type Posts = Vec<Post>;
pub type ResPosts = Vec<ResPost>;

pub trait PostsMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResPosts;
}

impl PostsMethods for Posts {
    fn to_res(&self) -> ResPosts {
        self.into_iter()
            .map(|post| post.to_res().unwrap())
            .collect()
    }
}
