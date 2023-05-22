// @generated automatically by Diesel CLI.

diesel::table! {
    registrations (id) {
        id -> Int4,
        instance_url -> Varchar,
        registration_id -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        redirect_uri -> Varchar,
        client_id -> Varchar,
        client_secret -> Varchar,
        vapid_key -> Nullable<Varchar>,
        nonce -> Varchar,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        username -> Varchar,
        access_token -> Varchar,
        token_type -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        created_at -> Nullable<Int4>,
        registration_id -> Int4,
        fail_count -> Nullable<Int4>,
        worker_id -> Int4,
    }
}

diesel::table! {
    workers (id) {
        id -> Int4,
    }
}

diesel::joinable!(tokens -> registrations (registration_id));
diesel::joinable!(tokens -> workers (worker_id));

diesel::allow_tables_to_appear_in_same_query!(
    registrations,
    tokens,
    workers,
);
