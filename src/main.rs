#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use request_mirror::schema::{history, pair_records};
use rocket::{data, request, Outcome, Request, Data};
use rocket::data::FromDataSimple;
use rocket::request::FromRequest;
use rocket_contrib::templates::Template;
use serde::Serialize;
use rocket::http::{Cookie, Cookies, Status};
use uuid::Uuid;
use std::io::Read;
use request_mirror::models::*;
use diesel::prelude::*;
use request_mirror::*;

#[derive(Serialize, Debug, Clone)]
enum PairType {
    Header,
    Cookie,
    Query,
    Body
}

#[derive(Serialize, Debug, Clone)]
struct Pair {
    key: String,
    value: String
}

#[derive(Serialize, Debug, Clone)]
struct RequestInfo {
    header: Vec<Pair>,
    cookies: Vec<Pair>,
    query: Vec<Pair>
}

#[derive(Serialize, Debug, Clone)]
struct RequestBody(String);

#[derive(Debug)]
enum ApiError {
}
#[derive(Serialize)]

struct ErrorContext {
    error_msg: String
}

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl<'a, 'r> FromRequest<'a, 'r> for RequestInfo {
    type Error = ApiError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut req_cookies = request.cookies();

        for row in req_cookies.iter() {
            println!("{}: {}", row.name(), row.value());
        }
        // Initially set cookie
        if req_cookies.get(&"mirror-id").is_none() {
            
            let new_uuid = Uuid::new_v4().to_string();

            println!("Creating new cookie");
            
            req_cookies.add(Cookie::new("mirror-id", new_uuid.clone()));
            
            let address = if request.client_ip().is_some() {
                request.client_ip().unwrap().to_string()
            } else {
                "Unknown".to_string()
            };
            
            let connection = &mut establish_connection();

            create_client(connection, &address, &new_uuid);
        }

        let mut header: Vec<Pair> = vec![];
        let mut cookies: Vec<Pair> = vec![];
        let mut query: Vec<Pair> = vec![];

        // Compile header
        for row in request.headers().clone().into_iter() {
            let key: String = row.name().to_string();
            let value: String = row.value().to_string();

            header.push(Pair{key:key, value:value});
        }

        // Compile cookies
        for row in req_cookies.iter() {
            let key: String = row.name().to_string();
            let value: String = row.value().to_string();

            cookies.push(Pair{key:key, value:value});
        }

        // Compile query
        let request_query = request.raw_query_items();
        if request_query.is_some() {
            for row in request_query.unwrap() {
                let (key, value) = row.key_value_decoded();

                query.push(Pair{key:key, value:value});
            }
        }

        Outcome::Success(RequestInfo{
            header: header,
            cookies: cookies,
            query: query 
        })
    }
}

impl FromDataSimple for RequestBody {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Return successfully.
        Outcome::Success(RequestBody(string))
    }
}

#[get("/")]
fn index(_info: RequestInfo) -> Template {
    
    //Redirect::to("/test")

    #[derive(Serialize)]
    struct Context{
    }
    
    Template::render("index", Context {})
}

#[get("/test")]
fn test_get(request: RequestInfo, cookies: Cookies) -> Template {

    println!("{request:?}");

    let cookie_id = cookies.get("mirror-id");

    if cookie_id.is_some() {
        let connection = &mut establish_connection();
        let history_id = create_history_record(connection, cookie_id.unwrap().value(), "Get");

        println!("Creating header Records for {history_id}");

        for row in &request.header {
            create_pair_record(connection, history_id, PairType::Header as i32, &row.key, &row.value);
        }

        println!("Creating cookie Records for {history_id}");

        for row in &request.cookies {
            create_pair_record(connection, history_id, PairType::Cookie as i32, &row.key, &row.value);
        }
        
        println!("Creating query Records for {history_id}");

        for row in &request.query {
            create_pair_record(connection, history_id, PairType::Query as i32, &row.key, &row.value);
        }

    }

    #[derive(Serialize)]
    struct Context<'a> {
        request_type: &'a str,
        header: Vec<Pair>,
        cookies: Vec<Pair>,
        query: Vec<Pair>,
    }

    let context = Context {
        request_type: "Get",
        header: request.header,
        cookies: request.cookies,
        query: request.query
    };

    Template::render("request_details", context)
}

#[post("/test", data = "<body>")]
fn test_post(body: RequestBody, request: RequestInfo, cookies: Cookies) -> Template {

    println!("{request:?}");

    println!("Input: {}", body.0);

    let cookie_id = cookies.get("mirror-id");

    if cookie_id.is_some() {
        let connection = &mut establish_connection();
        let history_id = create_history_record(connection, cookie_id.unwrap().value(), "Post");

        println!("Creating header Records for {history_id}");

        for row in &request.header {
            create_pair_record(connection, history_id, PairType::Header as i32, &row.key, &row.value);
        }

        println!("Creating cookie Records for {history_id}");

        for row in &request.cookies {
            create_pair_record(connection, history_id, PairType::Cookie as i32, &row.key, &row.value);
        }
        
        println!("Creating query Records for {history_id}");

        for row in &request.query {
            create_pair_record(connection, history_id, PairType::Query as i32, &row.key, &row.value);
        }

        println!("Creating body Records for {history_id}");
        create_pair_record(connection, history_id, PairType::Body as i32, "body", &body.0.clone());

    }

    #[derive(Serialize)]
    struct Context<'a> {
        request_type: &'a str,
        header: Vec<Pair>,
        cookies: Vec<Pair>,
        query: Vec<Pair>,
        body: String
    }

    let context = Context {
        request_type: "Post",
        header: request.header,
        cookies: request.cookies,
        query: request.query,
        body: body.0
    };

    Template::render("request_details", context)
}

#[get("/history")]
fn history_req(cookies: Cookies) -> Template {
    
    let cookie_id = cookies.get("mirror-id");

    let cookie_id = cookie_id.unwrap().value();
    println!("Client ID: {}", cookie_id);

    let connection = &mut establish_connection();
    let results = history::dsl::history
        .filter(history::client_id.eq(cookie_id.to_string()))
        .select(HistoryRecord::as_select())
        .load(connection)
        .expect("Error loading clients");

    for record in results.iter() {
        println!("{:?}", record);
    }

    #[derive(Serialize)]
    struct History {
        pub id: i32,
        pub client_id: String,
        pub request_type: String,
        pub timestamp: String
    }

    #[derive(Serialize)]
    struct Context {
        history_records: Vec<History>
    }

    let mut history_records: Vec<History> = Vec::new();

    for history_rec in results {
        history_records.push(
            History {
                id: history_rec.id,
                client_id: history_rec.client_id,
                request_type: history_rec.request_type,
                timestamp: history_rec.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
            }
        );
    }

    Template::render("history", Context{
        history_records: history_records
    })

}

#[get("/history/<history_id>")]
fn history_details(history_id: i32, cookies: Cookies) -> Template {
    
    let cookie_id = cookies.get("mirror-id");

    let cookie_id = cookie_id.unwrap().value();
    println!("Client ID: {}", cookie_id);

    let connection = &mut establish_connection();
    let results: Vec<HistoryRecord> = history::dsl::history
        .filter(history::id.eq(&history_id))
        .filter(history::client_id.eq(cookie_id.to_string()))
        .select(HistoryRecord::as_select())
        .load(connection)
        .expect("Error loading history records");

    if results.len() <= 0 {

        // Error
        return Template::render(
            "error",
            ErrorContext{ error_msg: "No Results Found. You may be unauthorized...".to_string() }
        );
    }

    let connection = &mut establish_connection();

    let pairs: Vec<PairRecord> = pair_records::dsl::pair_records
        .filter(pair_records::history_id.eq(history_id))
        .select(PairRecord::as_select())
        .load(connection)
        .expect("Error loading history records");

    let body: String = match &pairs.iter().filter(|res| res.pair_type == PairType::Body as i32).last() {
        Some(pair) => pair.value.clone(),
        _ => "".to_string()
    };

    let header: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Header as i32)
        .map(|res: &PairRecord| res)
        .collect();

    let cookies: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Cookie as i32)
        .map(|res: &PairRecord| res)
        .collect();

    let query: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Query as i32)
        .map(|res: &PairRecord| res)
        .collect();
    
    #[derive(Serialize)]
    struct Context<'a> {
        request_type: String,
        body: String,
        header: Vec<&'a PairRecord>,
        cookies: Vec<&'a PairRecord>,
        query: Vec<&'a PairRecord>,
    }

    Template::render(
        "request_details",
        Context{
            request_type: results[0].request_type.clone(),
            body: body.to_string(),
            header: header,
            cookies: cookies,
            query: query

    })

}


#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index, test_get, test_post, history_req, history_details])
    .attach(Template::fairing())
    .launch();
}