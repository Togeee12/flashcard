table! {
    users (unique_id) {
        unique_id -> VarChar,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        date_of_registration -> BigInt,
        country -> Varchar,
    }
}

table! {
    stacks (unique_id) {
        unique_id -> VarChar,
        owner_id -> Varchar,
        name -> VarChar,
        visibility -> Bool,
        cards_count -> Integer,
        tags -> Varchar,
    }
}

joinable!(stacks -> users (owner_id));

table! {
    cards (unique_id) {
        unique_id -> VarChar,
        stack_id -> Varchar,
        frontside -> Text,
        backside -> Text,
    }
}

joinable!(cards -> stacks (stack_id));
