pub mod models;
pub mod schema;

use chrono::offset::Utc;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{
    Client, NewClient, NewHistoryRecord, NewPairRecord
};
use schema::{
    clients, history, pair_records
};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_client(conn: &mut PgConnection, ip: &str, mirror_id: &str) -> usize {
    let new_client = NewClient {ip: &ip, mirror_id: &mirror_id};

    diesel::insert_into(clients::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .execute(conn)
        .expect("Error")
}

pub fn create_history_record(conn: &mut PgConnection, client_id: &str, request_type: &str, ) -> i32 {
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

pub fn create_pair_record(conn: &mut PgConnection, history_id: i32, pair_type: i32, key: &str, value: &str) -> usize {
    let new_pair_record = NewPairRecord {history_id: history_id, pair_type: pair_type, key: &key, value: &value};

    diesel::insert_into(pair_records::table)
        .values(&new_pair_record)
        .execute(conn).unwrap()
}
