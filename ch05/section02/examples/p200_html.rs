// Simple HTML Web Server
// Access at http://localhost:8080/

// Import necessary libraries from the Actix web framework
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template;

struct TodoEntry{
    id:u32,
    text:String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{
    entries:Vec<TodoEntry>,
}

// Define a custom error type `MyError` implementing the `ResponseError` trait
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for MyError {}

// Define a handler for the root path ("/") using the `get` attribute
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry{
        id:1,
        text:"First entry".to_string(),
    });
    entries.push(TodoEntry{
        id:2,
        text:"Second entry".to_string(),
    });

    let html=IndexTemplate{entries};

    // Define the response body content
    let response_body = html.render()?;

    // Return an HTTP response with the OK status and the defined body
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

// The main asynchronous function for running the Actix web server
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Configure and start the HTTP server with the defined handler
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")? // Bind the server to the specified IP address and port
        .run() // Run the server
        .await?; // Wait for the server to complete
    Ok(())
}
