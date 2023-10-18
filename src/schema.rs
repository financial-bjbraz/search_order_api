table! {
    account (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
     }
}

allow_tables_to_appear_in_same_query!(
    account,
    users,
);
