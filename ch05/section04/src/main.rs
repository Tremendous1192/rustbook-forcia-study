// Simple HTML Web Server
// Access at http://localhost:8080/

// Import necessary libraries from the Actix web framework
use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use thiserror::Error;

// Deserialize struct for adding todo items
#[derive(Deserialize)]
struct AddParams {
    text: String,
}

// Deserialize struct for deleting todo items
#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

// Struct representing a todo entry with id and text
struct TodoEntry {
    id: u32,
    text: String,
}

// Define a template for rendering the HTML page
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// Define a custom error type `MyError` implementing the `ResponseError` trait
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

// Handler for adding a todo item
#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>, // Extract form parameters from the request
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>, // Get the SQLite connection pool from the Actix web data
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?; // Acquire a connection from the connection pool
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?; // Execute an SQL insert statement
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish()) // Redirect to the root path after successful insertion
}

// Handler for adding a todo item
#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>, // Extract form parameters from the request
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>, // Get the SQLite connection pool from the Actix web data
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?; // Acquire a connection from the connection pool
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?; // Execute an SQL insert statement
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish()) // Redirect to the root path after successful insertion
}

// Handler for the root path ("/")
#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?; // Acquire a connection from the connection pool
    let mut statement = conn.prepare("SELECT id, text FROM todo")?; // Prepare an SQL query
    let rows = statement.query_map(params![], |row| {
        // Execute the query and map the results
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?); // Collect the mapped results into a vector
    }

    let html = IndexTemplate { entries };

    // Define the response body content
    let response_body = html.render()?; // Render the HTML template

    // Return an HTTP response with the OK status and the defined body
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// Main asynchronous function for running the Actix web server
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Initialize SQLite connection manager and connection pool
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    // Create a 'todo' table if it does not exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table 'todo'.");

    // Configure and start the HTTP server with the defined handler and connection pool
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")? // Bind the server to the specified IP address and port
    .run() // Run the server
    .await?; // Wait for the server to complete

    Ok(())
}
