mod status;

///
/// 主に開発環境でのテストデータ挿入で使用
///
pub fn insert_testdatas() {
    status::insert_statuses();
}

///
/// 主にステージング環境でのテストデータ挿入で使用
///
pub fn insert_testdatas_on_staging() {}

///
/// 主に本番環境でのテストデータ挿入で使用
///
pub fn insert_testdatas_on_master() {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn insert() {}
}
