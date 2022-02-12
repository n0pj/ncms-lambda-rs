use diesel::{Insertable, Queryable};

///
/// DB からデータを取得するための構造体
///
#[derive(Debug, Clone, Queryable)]
struct PostCategory {
    pub uuid: String,
}
