use diesel;
use diesel::prelude::*;
use diesel::dsl::exists;
use diesel::r2d2::{self, ConnectionManager};
use crate::{models, schema};

/// ## Alias for connection pool type
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type Conn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

/// ## Function for establishing a pool connection to the database
///
/// ### Returns
/// **db::Pool** type, aka **diesel::r2d2::Pool\<diesel::r2d2::ConnectionManager\<diesel::prelude::MysqlConnection\>\>**
pub fn establish_connection(url: String) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder().build(manager).unwrap_or_else(|err| {
        eprintln!("Couldn't create a db connection pool.:\n{}", err);
        std::process::exit(1);
    })
}

// --- managing users

/// ## Generates a free unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
///
/// ### Returns
/// Result containing unique id **String** or **diesel::result::Error**
pub fn generate_user_id(conn: &mut MysqlConnection) -> Result<String, diesel::result::Error> {
    let result = diesel::sql_query("SELECT generate_user_id() as unique_id")
        .get_result::<models::UniqueId>(conn);

    match result {
        Ok(unique_id) => Ok(unique_id.unique_id),
        Err(err) => Err(err)
    }
}

/// ## Checks if there is a user with given id
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `unique_id` - &str
///
/// ### Returns
/// Result containing **Bool** or **diesel::result::Error**
pub fn check_if_user_exists(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<bool, diesel::result::Error> {
    use schema::users::dsl::*;
    let exists_query = diesel::select(exists(users.filter(unique_id.eq(id))));
    
    match exists_query.get_result(conn) {
        Ok(result) => Ok(result),
        Err(diesel::result::Error::NotFound) => Ok(false), // Entry doesn't exist
        Err(err) => Err(err),
    }

}

/// ## Inserts a new user record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `user_to_insert` - &models::NewUser
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn add_user(
    conn: &mut MysqlConnection,
    user_to_insert: models::User,
) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;
    diesel::insert_into(users)
        .values(user_to_insert)
        .execute(conn)
}

/// ## Updates a user record.
/// 
/// This will select a record by it's unique_id and set all other collumns in the found record with parameters form card_to_update parameter.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `user_to_update` - &models::User
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn update_user(
    conn: &mut MysqlConnection,
    user_to_update: models::User,
) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;
    diesel::update(users.find(&user_to_update.unique_id))
        .set(&user_to_update)
        .execute(conn)
}

/// ## Selects a user record by it's unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing **models::User** or **diesel::result::Error**
pub fn get_user(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<models::User, diesel::result::Error> {
    use schema::users::dsl::*;
    users
        .find(id)
        .first::<models::User>(conn)
}

/// ## Selects a user record by email
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `email` - &str
///
/// ### Returns
/// Result containing **models::User** or **diesel::result::Error**
pub fn get_user_by_email(
    conn: &mut MysqlConnection,
    email: &str,
) -> Result<models::User, diesel::result::Error> {
    use schema::users::dsl;
    dsl::users
        .filter(dsl::email.eq(email))
        .first::<models::User>(conn)
}

/// ## Selects a user record by username
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `username` - &str
///
/// ### Returns
/// Result containing **models::User** or **diesel::result::Error**
pub fn get_user_by_username(
    conn: &mut MysqlConnection,
    username: &str,
) -> Result<models::User, diesel::result::Error> {
    use schema::users::dsl;
    dsl::users
        .filter(dsl::username.eq(username))
        .first::<models::User>(conn)
}

/// ## deletes a user record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn delete_user(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;
    diesel::delete(users.find(id))
        .execute(conn)
}

// --- managing stacks

/// ## Generates a free unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
///
/// ### Returns
/// Result containing unique id **String** or **diesel::result::Error**
pub fn generate_stack_id(conn: &mut MysqlConnection) -> Result<String, diesel::result::Error> {
    let result = diesel::sql_query("SELECT generate_stack_id() as unique_id")
        .get_result::<models::UniqueId>(conn);

    match result {
        Ok(unique_id) => Ok(unique_id.unique_id),
        Err(err) => Err(err)
    }
}

/// ## Inserts a new stack record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `stack_to_insert` - &models::NewStack
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn add_stack(
    conn: &mut MysqlConnection,
    stack_to_insert: models::Stack,
) -> Result<usize, diesel::result::Error> {
    use schema::stacks::dsl::*;
    diesel::insert_into(stacks)
        .values(stack_to_insert)
        .execute(conn)
}

/// ## Updates a stack record.
/// 
/// This will select a record by it's unique_id and set all other collumns in the found record with parameters form stack_to_update parameter.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `stack_to_update` - &models::Stack
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn update_stack(
    conn: &mut MysqlConnection,
    stack_to_update: models::Stack,
) -> Result<usize, diesel::result::Error> {
    use schema::stacks::dsl::*;
    diesel::update(stacks.find(&stack_to_update.unique_id))
        .set(&stack_to_update)
        .execute(conn)
}

/// ## Selects a stack record by it's unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing **models::Stack** or **diesel::result::Error**
pub fn get_stack(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<models::StackFull, diesel::result::Error> {
    use schema::stacks::dsl::*;
    stacks
        .find(id)
        .first::<models::StackFull>(conn)
}

/// ## Selects stack records by their owner id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing a Vec of **models::Stack** or **diesel::result::Error**
pub fn get_stacks_by_owner(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<Vec<models::StackFull>, diesel::result::Error> {
    use schema::stacks::dsl::*;
    stacks
        .filter(owner_id.eq(id))
        .load::<models::StackFull>(conn)
}

/// ## deletes a stack record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn delete_stack(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<usize, diesel::result::Error> {
    use schema::stacks::dsl::*;
    diesel::delete(stacks.find(id))
        .execute(conn)
}

// --- managing cards

/// ## Generates a free unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
///
/// ### Returns
/// Result containing unique id **String** or **diesel::result::Error**
pub fn generate_card_id(conn: &mut MysqlConnection) -> Result<String, diesel::result::Error> {
    let result = diesel::sql_query("SELECT generate_card_id() as unique_id")
        .get_result::<models::UniqueId>(conn);

    match result {
        Ok(unique_id) => Ok(unique_id.unique_id),
        Err(err) => Err(err)
    }
}

/// ## Inserts a new cards record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `card_to_insert` - &models::NewCard
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn add_card(
    conn: &mut MysqlConnection,
    card_to_insert: models::Card,
) -> Result<usize, diesel::result::Error> {
    use schema::cards::dsl::*;
    diesel::insert_into(cards)
        .values(card_to_insert)
        .execute(conn)
}

/// ## Updates a card record.
/// 
/// This will select a record by it's unique_id and set all other collumns in the found record with parameters form card_to_update parameter.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `card_to_update` - models::Card
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn update_card(
    conn: &mut MysqlConnection,
    card_to_update: models::Card,
) -> Result<usize, diesel::result::Error> {
    use schema::cards::dsl::*;
    diesel::update(cards.find(&card_to_update.unique_id))
        .set(&card_to_update)
        .execute(conn)
}

/// ## Selects a card record by it's unique id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing **models::Card** or **diesel::result::Error**
pub fn get_card(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<models::Card, diesel::result::Error> {
    use schema::cards::dsl::*;
    cards
        .find(id)
        .first::<models::Card>(conn)
}

/// ## Selects card records by their owner id.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing a Vec of **models::Card** or **diesel::result::Error**
pub fn get_cards_by_stack(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<Vec<models::Card>, diesel::result::Error> {
    use schema::cards::dsl::*;
    cards
        .filter(stack_id.eq(id))
        .load::<models::Card>(conn)
}

/// ## deletes a card record.
/// 
/// ### Arguments
///
/// * `conn` - &mut MysqlConnection
/// * `id` - &str
///
/// ### Returns
/// Result containing number of affected rows or **diesel::result::Error**
pub fn delete_card(
    conn: &mut MysqlConnection,
    id: &str,
) -> Result<usize, diesel::result::Error> {
    use schema::cards::dsl::*;
    diesel::delete(cards.find(id))
        .execute(conn)
}
