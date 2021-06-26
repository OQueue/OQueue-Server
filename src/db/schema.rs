table! {
    queue_entries (queue_id, user_id) {
        queue_id -> Uuid,
        user_id -> Uuid,
        order -> Int4,
        has_priority -> Bool,
        is_held -> Bool,
        joined_at -> Timestamp,
    }
}

table! {
    queues (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        organizer_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        exists_before -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        pwhash -> Bpchar,
    }
}

joinable!(queue_entries -> queues (queue_id));
joinable!(queue_entries -> users (user_id));

allow_tables_to_appear_in_same_query!(
    queue_entries,
    queues,
    users,
);
