use crate::schema::status;
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
pub struct Status {
    pub uuid: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Model<ResStatus, Error> for Status {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> Result<ResStatus, Error> {
        Ok(ResStatus {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// DB へデータを作成するための構造体
///
#[derive(Debug, Clone, Insertable)]
#[table_name = "status"]
struct NewStatus {
    pub uuid: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for NewStatus {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            uuid: Uuid::new_v4().to_string(),
            name: "".to_owned(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl NewModel<Status, NewStatus, Error> for NewStatus {
    fn from_model(model: &Status) -> Result<Self, Error> {
        Ok(Self {
            uuid: model.uuid.clone(),
            name: model.name.clone(),
            created_at: model.created_at.clone(),
            updated_at: model.updated_at.clone(),
        })
    }

    fn to_model(&self) -> Result<Status, Error> {
        Ok(Status {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn insert(&self) -> Result<Status, Error> {
        Ok(Status {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }

    fn update(&self) -> Result<Status, Error> {
        Ok(Status {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        })
    }
}

///
/// GraphQL のレスポンス用に変換
///
#[derive(Debug, Clone, GraphQLObject)]
struct ResStatus {
    pub uuid: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

pub type Statuses = Vec<Status>;
pub type ResStatuses = Vec<ResStatus>;

pub trait StatusesMethods {
    ///
    /// レスポンス用に変換
    ///
    fn to_res(&self) -> ResStatuses;
}

impl StatusesMethods for Statuses {
    fn to_res(&self) -> ResStatuses {
        self.into_iter()
            .map(|status| status.to_res().unwrap())
            .collect()
    }
}
