// Import the necessary Actix Web modules
use actix_web::{get, App, HttpResponse, HttpServer};

// Define a handler for the root path that returns a "Hello world!" response
#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";
    Ok(HttpResponse::Ok().body(response_body))
}

// Define the main asynchronous function for running the Actix Web server
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Create an Actix Web server that listens on all available network interfaces at port 8080
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")? // Bind the server to the specified IP address and port
        .run() // Run the server
        .await?; // Await the result
    Ok(()) // Return Ok to indicate successful execution
}