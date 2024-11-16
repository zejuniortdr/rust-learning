#[macro_use] extern crate rocket;

use rocket::{get, launch, routes, State};
use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use crate::models::AgendaItem;

mod models;

#[get("/agenda")]
async fn list_agenda(db: &State<Arc<Client>>) -> rocket::serde::json::Json<Vec<AgendaItem>> {
    let collection = db.database("agenda").collection::<AgendaItem>("items");
    let cursor = collection.find(None, None).await.unwrap();
    let results: Vec<_> = cursor.try_collect().await.unwrap();
    rocket::serde::json::Json(results)
}

#[launch]
async fn rocket() -> _ {
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = Arc::new(client);

    rocket::build()
        .manage(db)
        .mount("/", routes![list_agenda])
}
