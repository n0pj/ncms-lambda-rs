use crate::schema::post_comment;
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
pub struct PostComment {
    pub uuid: String,
    pub created_at: String,
    pub updated_at: String,
    pub content: String,
    pub post_uuid: String,
    pub user_uuid: String,
}

impl Model<ResPostComment, Error> for PostComment {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResPostComment, Error> {
        Ok(ResPostComment {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "post_comment"]
pub struct NewPostComment {
    pub uuid: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub post_uuid: String,
    pub user_uuid: String,
}

impl Default for NewPostComment {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            post_uuid: "".to_owned(),
            user_uuid: "".to_owned(),
            content: "".to_owned(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<PostComment, NewPostComment, Error> for NewPostComment {
    fn from_model(model: &PostComment) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            post_uuid: model.post_uuid.clone(),
            user_uuid: model.user_uuid.clone(),
            content: model.content.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<PostComment, Error> {
        Ok(PostComment {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn insert(&self) -> Result<PostComment, Error> {
        Ok(PostComment {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn update(&self) -> Result<PostComment, Error> {
        Ok(PostComment {
            uuid: self.uuid.clone(),
            post_uuid: self.post_uuid.clone(),
            user_uuid: self.user_uuid.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
pub struct ResPostComment {
    pub uuid: String,
    pub post_uuid: String,
    pub user_uuid: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub type PostComments = Vec<PostComment>;
pub type ResPostComments = Vec<ResPostComment>;

pub trait PostCommentsMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResPostComments;
}

impl PostCommentsMethods for PostComments {
    fn to_res(&self) -> ResPostComments {
        self.into_iter()
            .map(|post| post.to_res().unwrap())
            .collect()
    }
}
