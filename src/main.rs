mod services;
mod models;
mod types;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use services::{handler::TwitterLikeAPIHandler,config::service_config, api::TwitterLikeAPI};
use actix_web::{App, HttpServer,};




// main is the entry point of the program.
// It sets up an HTTP server using the Actix-Web framework and registers the routes and handlers defined in the services module.


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Instatiating a data structure(TwitterLikeAPI) with four fields: users, tweets, follows, and comments
  let data = TwitterLikeAPI{
    users: HashMap::new(),  // A hash map for storing user data
    tweets: HashMap::new(),  // A hash map for storing tweet data
    follows: HashMap::new(),  // A hash map for storing follow relationships between users
    comments: HashMap::new(),  // A hash map for storing comments on tweets
};

// Creates an atomic reference counted (Arc) pointer to a mutex (Mutex) containing the data
let api = Arc::new(Mutex::new(data));


    // Creates an HTTP server using the actix-web crate

    HttpServer::new(move || {
        App::new()
            // passing in a clone of the "api" variable as an argument
            .app_data(Data::new(TwitterLikeAPIHandler { api: api.clone() }))

            // Calls the "configure" function in the Services module, passing in the app instance
            .configure(service_config::configure)
    })
    // Binds the server to the "127.0.0.1:8080" address and starts the server using the run method
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


