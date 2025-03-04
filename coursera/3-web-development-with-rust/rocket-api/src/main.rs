extern crate rocket;
use crate::schema::users;

use rocket::{get, launch, routes};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
       .expect(&format!("Error connecting to {}", database_url))
}

#[macro_use]
extern crate diesel;

mod schema {
    table! {
        users (id) {
            id -> Int4,
            username -> Varchar,
            age -> Int4,
            email -> Varchar,
        }
    }
}


#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
struct User {
    username: String,
    age: i32,
    email: String,
}



#[get("/?<param1>&<param2>")]
fn index(param1: Option<String>, param2: Option<String>) -> String {
    let response = match (param1, param2) {
        (Some(p1), Some(p2)) => format!("Received param1: {}, param2: {}", p1, p2),
        (Some(p1), None) => format!("Received param1: {}, but param2 is missing", p1),
        (None, Some(p2)) => format!("Received param2: {}, but param1 is missing", p2),
        (None, None) => "No parameters provided".to_string(),
    };

    response
}

#[get("/health-check")]
fn health_check() -> &'static str {
    "Rocket API is running!"
}

#[get("/users/<id>")]
fn get_user(id: usize) -> String {
    format!("User with id: {}", id)
}


#[launch]
fn rocket() -> _ {
    println!("Starting server...");
    let mut conn = establish_connection();
    println!("Connected to database!");

    let new_user = User {
        username: "John Doe".to_string(),
        age: 30,
        email: "johndoe@example.com".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");


    diesel::update(users::table
        .filter(users::id.eq(2))) // Usando users::id para acessar a coluna 'id'
        .set(users::age.eq(31)) // Usando users::age para acessar a coluna 'age'
        .execute(&mut conn)
        .expect("Error updating user");

    let results = users::table
        .select((
            users::username,
            users::age,
            users::email,
        ))
        .filter(users::username.eq("John Doe")) // Usando users::name para acessar a coluna 'name'
        .load::<User>(&mut conn)
        .expect("Error retrieving user");

    // Imprimindo os resultados
    for user in results {
        println!("Name: {}, Age: {}, Email: {}", user.username, user.age, user.email);
    }

    diesel::update(users::table
        .filter(users::id.eq(2))) // Usando users::id para acessar a coluna 'id'
        .set(users::age.eq(31)) // Usando users::age para acessar a coluna 'age'
        .execute(&mut conn)
        .expect("Error updating user");

    diesel::delete(users::table.filter(users::id.eq(2))) // Usando users::id para acessar a coluna 'id'
        .execute(&mut conn)
        .expect("Error deleting user");

    rocket::build()
        .mount("/", routes![get_user, health_check, index])
}
