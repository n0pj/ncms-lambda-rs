table! {
    category (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        slug -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

table! {
    kind (uuid) {
        uuid -> Varchar,
        name -> Varchar,
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
        slug -> Varchar,
        updated_at -> Varchar,
        created_at -> Varchar,
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
        author -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        post_uuid -> Varchar,
    }
}

table! {
    post_kind (uuid) {
        uuid -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        post_uuid -> Varchar,
        kind_uuid -> Varchar,
    }
}

table! {
    post_score (uuid) {
        uuid -> Varchar,
        count -> Bigint,
        created_at -> Varchar,
        updated_at -> Varchar,
        post_score_kind_id -> Varchar,
        post_status_uuid -> Varchar,
    }
}

table! {
    post_score_kind (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        kind_uuid -> Varchar,
    }
}

table! {
    post_status (uuid) {
        uuid -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        status_uuid -> Varchar,
        post_uuid -> Varchar,
    }
}

table! {
    post_user (uuid) {
        uuid -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        user_uuid -> Varchar,
        post_uuid -> Varchar,
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
        name -> Varchar,
        email -> Varchar,
        display_name -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

joinable!(post_category -> category (category_uuid));
joinable!(post_category -> post (post_uuid));
joinable!(post_comment -> post (post_uuid));
joinable!(post_kind -> kind (kind_uuid));
joinable!(post_kind -> post (post_uuid));
joinable!(post_score -> post_score_kind (post_score_kind_id));
joinable!(post_score -> post_status (post_status_uuid));
joinable!(post_score_kind -> kind (kind_uuid));
joinable!(post_status -> post (post_uuid));
joinable!(post_status -> status (status_uuid));
joinable!(post_user -> post (post_uuid));
joinable!(post_user -> user (user_uuid));

allow_tables_to_appear_in_same_query!(
    category,
    kind,
    post,
    post_category,
    post_comment,
    post_kind,
    post_score,
    post_score_kind,
    post_status,
    post_user,
    status,
    user,
);
