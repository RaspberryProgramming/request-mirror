pub mod models;
pub mod schema;

use chrono::offset::Utc;
use diesel::prelude::*;
use rocket::http::CookieJar;
use std::env;
use dotenvy::dotenv;
use models::{
    Client,
    NewClient,
    NewHistoryRecord,
    NewPairRecord,
    PairType,
    Ownership
};
use schema::{
    clients,
    history,
    pair_records,
    ownership
};

/// Establishes diesel Postgres connection that can be used to iteract with the database
/// 
/// Example:
/// 
/// ```rust,ignore
/// use request_mirror::establish_connection;
/// 
/// let connection = establish_connection();
/// ```
pub fn establish_connection() -> PgConnection {
    
    let key: &str = "DATABASE_URL";
    let database_url: String;
    match env::var(key) {
        Ok(val) => database_url = val,
        Err(_e) => {
            dotenv().ok();

            database_url = env::var(key).expect("DATABASE_URL must be set");
        }
    }
        
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// returns default cookie expiration in weeks
/// 
/// This value can be set by modifying an environment variable or setting in the .env file.
/// 
/// Example:
/// 
/// ```rust,ignore
/// use request_mirror::establish_connection;
/// 
/// let connection = establish_connection();
/// ```
pub fn cookie_expiration() -> i64 {
    
    let key: &str = "COOKIE_EXPIRATION";
    let expiration: i64;
    match env::var(key) {
        Ok(val) => {
            match val.parse::<i64>() {
                Ok(v) => expiration = v,
                Err(e) => expiration = 52
            }
        },
        Err(_e) => {
            dotenv().ok();

            expiration = 52;
        }
    }
        
    expiration
}

/// Used to create a new client in the database. You need to pass a connection, the ip and client_id
/// 
/// Example:
/// 
/// ```rust,ignore
/// use request_mirror::{establish_connection, create_client};
/// 
/// let mut connection = establish_connection();
/// 
/// create_client(&mut connection, "192.168.0.1", "195222-222-123123");
/// ```
/// 
/// create_client returns a value of usize which represents the number of entries created
pub fn create_client(conn: &mut PgConnection, ip: &str, client_id: &str) -> usize {
    let new_client = NewClient {ip: &ip, client_id: &client_id};

    diesel::insert_into(clients::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .execute(conn)
        .expect("Error: Couldn't create a new client in the database.")
}

/// Creates a new history record in the database. A history record is the initial record to store a
/// request that is sent to the application.
/// The history record has a client_id, request_type, and a timestamp.
pub fn create_history_record(conn: &mut PgConnection, client_id: &str, request_type: &str, ) -> i64 {
    let new_history_record = NewHistoryRecord {
        client_id: &client_id,
        request_type: request_type,
        timestamp: Utc::now().naive_utc()
    };

    diesel::insert_into(history::table)
        .values(&new_history_record)
        .returning(history::columns::id)
        //.execute(conn).unwrap()       
        .get_result(conn).unwrap()
}

/// Creates a new pair record in the database.
/// Pass the history_id that this pair will be associated, the PairType, key and value of the pair
/// pair_type is automatically casted as i32 type
pub fn create_pair_record(conn: &mut PgConnection, history_id: i64, pair_type: PairType, key: &str, value: &str) -> usize {
    let new_pair_record = NewPairRecord {history_id: history_id, pair_type: pair_type as i16, key: &key, value: &value};

    diesel::insert_into(pair_records::table)
        .values(&new_pair_record)
        .execute(conn).unwrap()
}

/// Creates a new ownership record in the database
/// Pass the owner_id and client_id
pub fn create_owner_record(conn: &mut PgConnection, owner_id: String, client_id: String) -> usize {
    let new_ownership_record = Ownership {
        owner_id,
        client_id
    };

    diesel::insert_into(ownership::table)
        .values(&new_ownership_record)
        .execute(conn).unwrap()
}

/// retrieve a Vec of ownership relationships which client_id owns
pub fn get_ownerships(client_id: &str, connection: &mut PgConnection) -> Vec<Ownership> {
    ownership::dsl::ownership
        .filter(ownership::owner_id.eq(client_id))
        .select(Ownership::as_select())
        .load(connection)
        .expect("Error loading history records")
}

/// Returns boolean for whether the given ownership exists between client_id and owner_id
pub fn ownership_exists(client_id: &str, owner_id: &str, connection: &mut PgConnection) -> bool {
    let ownerships = ownership::dsl::ownership
        .filter(ownership::client_id.eq(client_id).and(ownership::owner_id.eq(owner_id)))
        .select(Ownership::as_select())
        .load(connection)
        .expect("Error loading history records");

    ownerships.len() > 0
}

/// Looks for a cookie in the cookiejar. If it is a pending cookie, the pending will be returned as a String.
/// This accounts for if this client hasn't connected before and is making its first request
pub fn get_cookie(key: &str, cookies: &CookieJar<'_>) -> String {
    match cookies.get_pending(key) {
        Some(v) => v.value().to_string(),
        None => cookies.get(key).unwrap().value().to_string()
    }
}