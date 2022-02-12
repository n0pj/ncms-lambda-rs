use crate::models::status::NewStatus;
use crate::schema::status::dsl::status;
use chrono::Utc;
use diesel::prelude::*;
use ncms_core::db::mysql::establish_connection;
use uuid::Uuid;

pub fn insert_statuses() {
    let conn = establish_connection();
    let statuses = vec![
        NewStatus {
            uuid: Uuid::new_v4().to_string(),
            name: "draft".to_string(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        },
        NewStatus {
            uuid: Uuid::new_v4().to_string(),
            name: "published".to_string(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        },
    ];

    diesel::insert_into(status)
        .values(&statuses)
        .execute(&conn)
        .expect("Error saving new status");

    println!("Inserted {} status", statuses.len());
}
