use crate::{schema::*, db};
use argon2::Argon2;

use diesel;
use diesel::prelude::*;
use diesel::sql_types::VarChar;


#[derive(Debug, Clone)]
pub struct AppData<'a> {
    pub pool: db::Pool,
    pub jwt_secret: String,
    pub jwt_duration: u64,
    pub domain: String,
    pub argon2: Argon2<'a>,
}

// --- Diesel

#[derive(Debug, QueryableByName)]
pub struct UniqueId {
    #[diesel(sql_type = VarChar)]
    pub unique_id: String,
}


#[derive(Debug, Clone, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub unique_id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub date_of_registration: i64,
    pub country: String,
}


#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = stacks)]
pub struct Stack {
    pub unique_id: String,
    pub owner_id: String,
    pub name: String,
    pub visibility: bool,
    pub tags: String,
}

impl From<StackFull> for Stack {
    fn from(stack_full: StackFull) -> Self {
        Stack {
            unique_id: stack_full.unique_id,
            owner_id: stack_full.owner_id,
            name: stack_full.name,
            visibility: stack_full.visibility,
            tags: stack_full.tags,
        }
    }
}

#[derive(Debug, Clone, Queryable)]
pub struct StackFull {
    pub unique_id: String,
    pub owner_id: String,
    pub name: String,
    pub visibility: bool,
    pub cards_count: i32,
    pub tags: String,
}


#[derive(Debug, Clone, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = cards)]
pub struct Card {
    pub unique_id: String,
    pub stack_id: String,
    pub frontside: String,
    pub backside: String,
}