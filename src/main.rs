mod api;
mod database;
pub mod models;
pub mod schema;
mod utils;

use actix_web::{dev::ServiceRequest,Error,get, middleware::Logger, web, web::Data, App, HttpServer, Responder};
use actix_web_httpauth::{extractors::bearer::BearerAuth,middleware::HttpAuthentication};

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;

use std::sync::{Arc,Mutex};

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, Error> {
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv::dotenv().ok();

    let client = BasicClient::new(
        ClientId::new(dotenv::var("DISCORD_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(
            dotenv::var("DISCORD_CLIENT_SECRET").unwrap(),
        )),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(dotenv::var("DISCORD_REDIRECT_URI").unwrap()).unwrap());

    HttpServer::new(move || {
        let database = database::Database::new();
        App::new()
            .wrap(Logger::default())
            .wrap(HttpAuthentication::bearer(validator))
            .service(api::activity)
            .app_data(Data::new(database))
    })
    .bind("localhost:8000")?
    .run()
    .await
}
