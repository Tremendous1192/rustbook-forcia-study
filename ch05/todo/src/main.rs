// Import the necessary Actix Web modules
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};

// Import the `askama` crate for templates and the `thiserror` crate for error handling
use askama::Template;
use thiserror::Error;

// Define a struct for representing a to-do entry
struct TodoEntry {
    id: u32,
    text: String,
}

// Define a struct for the HTML template using the `askama` template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// Define a custom error type `MyError` using the `thiserror` crate
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

// Implement the `ResponseError` trait for the custom error type
impl ResponseError for MyError {}

// Define a handler for the root path that returns a result with `MyError` as the error type
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    // Create a list of to-do entries
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    // Render the HTML template using `askama`
    let html = IndexTemplate { entries };
    let response_body = html.render()?; // Render the template and handle any errors

    // Return an HTTP response with the rendered HTML
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// Define the main asynchronous function for running the Actix Web server
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Create an Actix Web server that listens on all available network interfaces at port 8080
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
