use crate::models::user::ResUser;
use chrono::Utc;
use serde::Serialize;

// pub struct Mfa {
//     pub uuid: String,
//     pub password: String,
//     pub expired_at: String,
//     pub created_at: String,
//     pub updated_at: String,
//     pub user_uuid: String,
// }

#[derive(Debug, Clone)]
pub struct NewMfa {
    pub qr_code_url: String,
    pub created_at: String,
}

impl Default for NewMfa {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            qr_code_url: "".to_owned(),
            created_at: now,
        }
    }
}

/// レスポンス
#[derive(GraphQLObject, Clone)]
pub struct ResMfa {
    pub user: ResUser,
    pub qr_code_url: String,
    pub created_at: String,
}

impl Default for ResMfa {
    fn default() -> Self {
        let now = Utc::now().to_string();

        Self {
            user: ResUser::default(),
            qr_code_url: "".to_owned(),
            created_at: now,
        }
    }
}
