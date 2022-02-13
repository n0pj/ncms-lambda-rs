table! {
    category (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        slug -> Varchar,
        is_main -> Bool,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

table! {
    post (uuid) {
        uuid -> Varchar,
        title -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        content -> Longtext,
        slug -> Nullable<Varchar>,
        created_at -> Varchar,
        updated_at -> Varchar,
        status_uuid -> Varchar,
        user_uuid -> Varchar,
    }
}

table! {
    post_category (uuid) {
        uuid -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        post_uuid -> Varchar,
        category_uuid -> Varchar,
    }
}

table! {
    post_comment (uuid) {
        uuid -> Varchar,
        content -> Longtext,
        created_at -> Varchar,
        updated_at -> Varchar,
        post_uuid -> Varchar,
        user_uuid -> Varchar,
    }
}

table! {
    session (uuid) {
        uuid -> Varchar,
        token_secret -> Varchar,
        bearer_token -> Varchar,
        expired_at -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        user_uuid -> Varchar,
    }
}

table! {
    status (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

table! {
    user (uuid) {
        uuid -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        google_authenticator_secret -> Nullable<Varchar>,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

joinable!(post -> status (status_uuid));
joinable!(post -> user (user_uuid));
joinable!(post_category -> category (category_uuid));
joinable!(post_category -> post (post_uuid));
joinable!(post_comment -> post (post_uuid));
joinable!(post_comment -> user (user_uuid));
joinable!(session -> user (user_uuid));

allow_tables_to_appear_in_same_query!(
    category,
    post,
    post_category,
    post_comment,
    session,
    status,
    user,
);
