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
struct RequestInfo {
    header: Vec<Pair>,
    cookies: Vec<Pair>,
    query: Vec<Pair>
}

#[derive(Debug)]
enum ApiError {
}

#[derive(Serialize, Debug, Clone)]
struct RequestBody(String);

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl<'a, 'r> FromRequest<'a, 'r> for RequestInfo {
    type Error = ApiError;

    /// Used for parsing request information and making it available for request functions to access
    /// Also handles creating cookies when the client doesn't send one in the request
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut req_cookies = request.cookies();

        // Initially set cookie
        if req_cookies.get(&"mirror-id").is_none() {
            
            // When the client doesn't send the mirror-id cookie, from_request will create
            // one in the database and send it back to the client.
            let new_uuid = Uuid::new_v4().to_string();

            println!("Creating new cookie");
            
            req_cookies.add(Cookie::new("mirror-id", new_uuid.clone()));
            
            let address = if request.client_ip().is_some() {
                request.client_ip().unwrap().to_string()
            } else {
                "Unknown".to_string()
            };
            
            let connection = &mut establish_connection();

            // Creates a new client record in the database
            create_client(connection, &address, &new_uuid);
        }

        // Compile vector of Pair structs with each header, cookie and query param that is coming
        // from the client
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

        // Send the Request Info as successful outcome
        Outcome::Success(RequestInfo {
            header: header,
            cookies: cookies,
            query: query 
        })
    }
}

impl FromDataSimple for RequestBody {
    type Error = String;

    /// Used to extract the body of a request and make it available to request functions
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

/// Returns the index template
#[get("/")]
fn index(_info: RequestInfo) -> Template {
    
    #[derive(Serialize)]
    struct Context{
    }
    
    Template::render("index", Context {})
}

/// Processes /test get request and responds with some of the contents of the request
/// This function also stores information from the request in the database
#[get("/test")]
fn test_get(request: RequestInfo, cookies: Cookies) -> Template {

    let client_id = cookies.get("mirror-id");

    // If the cookie exists, create new database records for this request
    if client_id.is_some() {
        let connection = &mut establish_connection();

        // Create new history record and retrieve new history_id
        let history_id = create_history_record(connection, client_id.unwrap().value(), "Get");

        // Create header pair records
        for row in &request.header {
            create_pair_record(connection, history_id, PairType::Header, &row.key, &row.value);
        }

        // Create cookie pair records
        for row in &request.cookies {
            create_pair_record(connection, history_id, PairType::Cookie, &row.key, &row.value);
        }
        
        // Create query pair records
        for row in &request.query {
            create_pair_record(connection, history_id, PairType::Query, &row.key, &row.value);
        }

    }

    // Define context for the template
    #[derive(Serialize)]
    struct Context<'a> {
        request_type: &'a str,
        header: Vec<Pair>,
        cookies: Vec<Pair>,
        query: Vec<Pair>,
    }

    // Compile context
    let context = Context {
        request_type: "Get",
        header: request.header,
        cookies: request.cookies,
        query: request.query
    };

    // Render request_details
    Template::render("request_details", context)
}

/// Processes /test post request and responds with some of the contents of the request
/// This function also stores information from the request in the database
#[post("/test", data = "<body>")]
fn test_post(body: RequestBody, request: RequestInfo, cookies: Cookies) -> Template {

    let client_id = cookies.get("mirror-id");
    
    // If the cookie exists, create new database records for this request
    if client_id.is_some() {
        let connection = &mut establish_connection();
        
        // Create new history record and retrieve new history_id
        let history_id = create_history_record(connection, client_id.unwrap().value(), "Post");
        
        // Create header pair records
        for row in &request.header {
            create_pair_record(connection, history_id, PairType::Header, &row.key, &row.value);
        }

        // Create cookie pair records
        for row in &request.cookies {
            create_pair_record(connection, history_id, PairType::Cookie, &row.key, &row.value);
        }
        
         // Create query pair records
        for row in &request.query {
            create_pair_record(connection, history_id, PairType::Query, &row.key, &row.value);
        }

        // Create a pair record for body of the request
        println!("Creating body Records for {history_id}");
        create_pair_record(connection, history_id, PairType::Body, "body", &body.0.clone());

    }

    // Define context for the template
    #[derive(Serialize)]
    struct Context<'a> {
        request_type: &'a str,
        header: Vec<Pair>,
        cookies: Vec<Pair>,
        query: Vec<Pair>,
        body: String
    }

    // Compile context
    let context = Context {
        request_type: "Post",
        header: request.header,
        cookies: request.cookies,
        query: request.query,
        body: body.0
    };

    // Render request_details Template
    Template::render("request_details", context)
}

/// Request function that returns a history of requests that the current client has made
/// The user can click a history_id and view the request itself
#[get("/history")]
fn history_req(cookies: Cookies) -> Template {
    
    // Get the client_id from the cookies
    let client_id = cookies.get("mirror-id").unwrap().value();

    let connection = &mut establish_connection();

    // Query the database for history records
    let results = history::dsl::history
        .filter(history::client_id.eq(client_id.to_string()))
        .select(HistoryRecord::as_select())
        .load(connection)
        .expect("Error loading clients");

    /// Template Context
    #[derive(Serialize)]
    struct Context {
        history_records: Vec<History>
    }

    // New vector to store converted History structs
    let mut history_records: Vec<History> = Vec::new();

    // For each HistoryRecord, create a new History struct
    for history_rec in results {
        history_records.push(
            History {
                id: history_rec.id,
                client_id: history_rec.client_id,
                request_type: history_rec.request_type,
                timestamp: history_rec.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string() // convert to string
            }
        );
    }

    // Render the template
    Template::render(
        "history",
        Context{
            history_records: history_records
        }
    )

}

/// The history_details request function will provide a webpage that a user can view the contents of a 
/// request that was recorded to the database.
/// This includes the body of a post request, headers, cookies and query parameters.
#[get("/history/<history_id>")]
fn history_details(history_id: i64, cookies: Cookies) -> Template {
    
    let client_id = cookies.get("mirror-id").unwrap().value();

    let connection = &mut establish_connection();

    // Query history table where the history id is what was in the route. Also filter client_ids.
    // If this record has a different client_id nothing will be returned
    let results: Vec<HistoryRecord> = history::dsl::history
        .filter(history::id.eq(&history_id))
        .filter(history::client_id.eq(client_id.to_string()))
        .select(HistoryRecord::as_select())
        .load(connection)
        .expect("Error loading history records");

    if results.len() <= 0 {

        // Error when no results are returned
        return Template::render(
            "error",
            ErrorContext{ error_msg: "No Results Found. You may be not be authorized to view this record...".to_string() }
        );
    }

    let connection = &mut establish_connection();

    // Query all Pair Records for this history_id
    let pairs: Vec<PairRecord> = pair_records::dsl::pair_records
        .filter(pair_records::history_id.eq(history_id))
        .select(PairRecord::as_select())
        .load(connection)
        .expect("Error loading history records");

    // Filter Body Pair Records for this history_id
    let body: String = match &pairs.iter()
        .filter(|res| res.pair_type == PairType::Body as i16).last() // Filter and get last record
    {
        Some(pair) => pair.value.clone().to_string(), // Return pair value if Some
        _ => "".to_string() // Return empty string if None
    };

    // Collect Header Pair Records into a vector
    let header: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Header as i16)
        .map(|res: &PairRecord| res)
        .collect();

    // Collect Cookie Pair Records into a vector
    let cookies: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Cookie as i16)
        .map(|res: &PairRecord| res)
        .collect();
    
    // Collect Query Pair Records into a vector
    let query: Vec<&PairRecord> = pairs.iter()
        .filter(|res: &&PairRecord| res.pair_type == PairType::Query as i16)
        .map(|res: &PairRecord| res)
        .collect();
    
    /// history_details specific context
    #[derive(Serialize)]
    struct Context<'a> {
        request_type: String,
        body: String,
        header: Vec<&'a PairRecord>,
        cookies: Vec<&'a PairRecord>,
        query: Vec<&'a PairRecord>,
    }

    // Render request_details with data taken from the database
    Template::render(
        "request_details",
        Context{
            request_type: "Previous ".to_owned() + &results[0].request_type.clone(),
            body: body,
            header: header,
            cookies: cookies,
            query: query

    })

}

/// 404 Response
#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "error",
        ErrorContext{
            error_msg: format!("Oh no! We couldn't find the requested path '{}'", req.uri())
        }
    )
}

fn main() {
    rocket::ignite()
    .register(catchers![not_found])
    .mount(
        "/",
        routes![
            index,
            test_get,
            test_post,
            history_req,
            history_details
        ]
    )
    .attach(Template::fairing())
    .launch();
}