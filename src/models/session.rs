use crate::schema::session;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::{insert_into, Insertable, Queryable};
use juniper::GraphQLObject;
use ncms_core::{db::mysql::establish_connection, gen_secret};
pub use ncms_core::{Model, NewModel};
use std::io::{Error, ErrorKind};
use uuid::Uuid;

///
/// DB からデータを取得するための構造体
///
#[derive(Debug, Clone, Queryable)]
pub struct Session {
    pub uuid: String,
    pub token_secret: String,
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
            // token_secret: self.token_secret.clone(),
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
    pub token_secret: String,
    pub bearer_token: String,
    pub expired_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_uuid: String,
}

impl Default for NewSession {
    fn default() -> Self {
        let now = Utc::now();
        let expired_at = (now.clone() + Duration::days(30)).to_string();
        let now = now.to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            user_uuid: "".to_owned(),
            token_secret: gen_secret(255),
            bearer_token: "".to_owned(),
            expired_at,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<Session, NewSession, Error> for NewSession {
    fn from_model(model: &Session) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            token_secret: model.token_secret.clone(),
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
            token_secret: self.token_secret.clone(),
            bearer_token: self.bearer_token.clone(),
            expired_at: self.expired_at.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_uuid: self.user_uuid.clone(),
        })
    }

    fn insert(&self) -> Result<Session, Error> {
        use crate::schema::session::dsl::*;

        let conn = establish_connection();
        let result = insert_into(session).values(self).execute(&conn);

        match result {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
        }

        Ok(self.to_model()?)
    }

    fn update(&self) -> Result<Session, Error> {
        Ok(Session {
            uuid: self.uuid.clone(),
            token_secret: self.token_secret.clone(),
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
    // pub token_secret: String,
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
