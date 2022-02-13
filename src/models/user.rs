use crate::schema::user;
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
pub struct User {
    pub uuid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub google_authenticator_secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Model<ResUser, Error> for User {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResUser, Error> {
        Ok(ResUser {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            password: self.password.clone(),
            google_authenticator_secret: self.google_authenticator_secret.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub uuid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub google_authenticator_secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for NewUser {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            name: None,
            email: None,
            display_name: None,
            password: None,
            google_authenticator_secret: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<User, NewUser, Error> for NewUser {
    fn from_model(model: &User) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            name: None,
            email: None,
            display_name: None,
            password: None,
            google_authenticator_secret: None,
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<User, Error> {
        Ok(User {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            password: self.password.clone(),
            google_authenticator_secret: None,
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn insert(&self) -> Result<User, Error> {
        Ok(User {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            password: self.password.clone(),
            google_authenticator_secret: None,
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn update(&self) -> Result<User, Error> {
        Ok(User {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            password: self.password.clone(),
            google_authenticator_secret: None,
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
pub struct ResUser {
    pub uuid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub google_authenticator_secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for ResUser {
    fn default() -> Self {
        Self {
            uuid: "".to_string(),
            name: None,
            email: None,
            display_name: None,
            password: None,
            google_authenticator_secret: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }
}

pub type Users = Vec<User>;
pub type ResUsers = Vec<ResUser>;

pub trait UserssMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResUsers;
}

impl UserssMethods for Users {
    fn to_res(&self) -> ResUsers {
        self.into_iter()
            .map(|user| user.to_res().unwrap())
            .collect()
    }
}
