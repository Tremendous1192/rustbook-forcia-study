// Simple Web Server
// Access at http://localhost:8080/

// Import necessary libraries from the Actix web framework
use actix_web::{get, App, HttpResponse, HttpServer};

// Define a handler for the root path ("/") using the `get` attribute
#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    // Define the response body content
    let response_body = "Hello world!";
    // Return an HTTP response with the OK status and the defined body
    Ok(HttpResponse::Ok().body(response_body))
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
