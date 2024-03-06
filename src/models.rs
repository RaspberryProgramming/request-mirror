use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::{
    clients,
    history,
    pair_records
};
use diesel::pg::Pg;

/// Client Record that keeps track of unique clients that have connected to this application
/// These are used to keep track of who submitted what request that will be recorded
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(Pg))]
pub struct Client {
    pub id: i64,
    pub ip: String,
    pub client_id: String,
}

/// Used to create a new Client Record
#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient<'a> {
    pub ip: &'a str,
    pub client_id: &'a str,
}

/// History record that keeps track of each unique request that comes in to the /test route
/// Any request is recorded here and the additional attributes are stored as a Pair Record
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = history)]
#[diesel(check_for_backend(Pg))]
pub struct HistoryRecord {
    pub id: i64,
    pub client_id: String,
    pub request_type: String,
    pub timestamp: NaiveDateTime
}

/// History Struct used to represent HistoryRecord in a serializable way.
/// This is intended to allow an instance to be used with a Handlebars Template
#[derive(Serialize)]
pub struct History {
    pub id: i64,
    pub client_id: String,
    pub request_type: String,
    pub timestamp: String
}

/// Used to create a new History Record
#[derive(Insertable)]
#[diesel(table_name = history)]
pub struct NewHistoryRecord<'a> {
    pub client_id:  &'a str,
    pub request_type: &'a str,
    pub timestamp: NaiveDateTime
}

/// Pair Records are used to store attributes of a request. This includes a body record, header, query and cookie record.
/// Each has a key and value.
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = pair_records)]
#[diesel(check_for_backend(Pg))]
pub struct PairRecord {
    pub id: i64,
    pub history_id: i64,
    pub pair_type: i16,
    pub key: String,
    pub value: String
}

/// Used to create a new Pair Record
#[derive(Insertable)]
#[diesel(table_name = pair_records)]
pub struct NewPairRecord<'a> {
    pub history_id: i64,
    pub pair_type: i16,
    pub key: &'a str,
    pub value: &'a str
}

/// Used to indicate the type of pair a Pair Record may be.
#[derive(Serialize, Debug, Clone)]
pub enum PairType {
    Header,
    Cookie,
    Query,
    Body
}

/// Used to store a pair of key values
/// This structure is used for storing different values in the database and
/// rendering mustache templates with an array of key value pairs.
#[derive(Serialize, Debug, Clone)]
pub struct Pair {
    pub key: String,
    pub value: String
}

/// Used when needing to return a template with an error message.
/// Specify error_msg to display your custom message.
#[derive(Serialize)]
pub struct ErrorContext {
    pub error_msg: String
}