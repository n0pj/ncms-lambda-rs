use crate::schema::session;
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
pub struct Session {
    pub uuid: String,
    pub bearer_token: String,
    pub expired_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_uuid: String,
}

impl Model<ResSession, Error> for Session {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResSession, Error> {
        Ok(ResSession {
            uuid: self.uuid.clone(),
            bearer_token: self.bearer_token.clone(),
            expired_at: self.expired_at.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_uuid: self.user_uuid.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "session"]
pub struct NewSession {
    pub uuid: String,
    pub bearer_token: String,
    pub expired_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_uuid: String,
}

impl Default for NewSession {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            user_uuid: "".to_owned(),
            bearer_token: "".to_owned(),
            expired_at: now.clone(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<Session, NewSession, Error> for NewSession {
    fn from_model(model: &Session) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            bearer_token: model.bearer_token.clone(),
            expired_at: model.expired_at.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
            user_uuid: model.user_uuid.clone(),
        })
    }

    fn to_model(&self) -> Result<Session, Error> {
        Ok(Session {
            uuid: self.uuid.clone(),
            bearer_token: self.bearer_token.clone(),
            expired_at: self.expired_at.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_uuid: self.user_uuid.clone(),
        })
    }

    fn insert(&self) -> Result<Session, Error> {
        Ok(Session {
            uuid: self.uuid.clone(),
            bearer_token: self.bearer_token.clone(),
            expired_at: self.expired_at.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_uuid: self.user_uuid.clone(),
        })
    }

    fn update(&self) -> Result<Session, Error> {
        Ok(Session {
            uuid: self.uuid.clone(),
            bearer_token: self.bearer_token.clone(),
            expired_at: self.expired_at.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_uuid: self.user_uuid.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
pub struct ResSession {
    pub uuid: String,
    pub bearer_token: String,
    pub expired_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_uuid: String,
}

pub type Sessions = Vec<Session>;
pub type ResSessions = Vec<ResSession>;

pub trait SessionsMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResSessions;
}

impl SessionsMethods for Sessions {
    fn to_res(&self) -> ResSessions {
        self.into_iter()
            .map(|session| session.to_res().unwrap())
            .collect()
    }
}
