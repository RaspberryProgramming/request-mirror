use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::{
    clients,
    history,
    pair_records
};
use diesel::pg::Pg;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(Pg))]
pub struct Client {
    pub id: i32,
    pub ip: String,
    pub mirror_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient<'a> {
    pub ip: &'a str,
    pub mirror_id: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = history)]
#[diesel(check_for_backend(Pg))]
pub struct HistoryRecord {
    pub id: i32,
    pub client_id: String,
    pub request_type: String,
    pub timestamp: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = history)]
pub struct NewHistoryRecord<'a> {
    pub client_id:  &'a str,
    pub request_type: &'a str,
    pub timestamp: NaiveDateTime
}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = pair_records)]
#[diesel(check_for_backend(Pg))]
pub struct PairRecord {
    pub id: i32,
    pub history_id: i32,
    pub pair_type: i32,
    pub key: String,
    pub value: String
}

#[derive(Insertable)]
#[diesel(table_name = pair_records)]
pub struct NewPairRecord<'a> {
    pub history_id: i32,
    pub pair_type: i32,
    pub key: &'a str,
    pub value: &'a str
}