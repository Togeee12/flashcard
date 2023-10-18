use diesel;
use diesel::prelude::*;
use diesel::sql_types::VarChar;
use crate::schema::*;


#[derive(Debug, QueryableByName)]
pub struct UniqueId {
    #[sql_type = "VarChar"]
    pub unique_id: String,
}


#[derive(Debug, Clone, Queryable, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub unique_id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub date_of_registration: i64,
    pub country: String,
}


#[derive(Insertable, AsChangeset)]
#[table_name = "stacks"]
pub struct Stack {
    pub unique_id: String,
    pub owner_id: String,
    pub name: String,
    pub visibility: bool,
    pub tags: String,
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
#[table_name = "cards"]
pub struct Card {
    pub unique_id: String,
    pub stack_id: String,
    pub frontside: String,
    pub backside: String,
}