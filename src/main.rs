use std::{env, time::Duration};

use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Json, Router};
use db::{add_user_to_db, get_all_users, User};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
mod db;
#[derive(Serialize, Deserialize, Debug,Hash)]
struct Response{
    message:String
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let database_url = env::var("DB_URL").expect("Database connection string is missing");
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await
    .expect("Unable to connect to the database");

    let app = Router::new().route("/",get(hello_world))
    .route("/users", get(get_users))
    .route("/user",post(add_user))
    .with_state(pool);
    let app = app.fallback(handler_404);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
   .await
   .unwrap();
   println!("listening on {}", listener.local_addr().unwrap());
   axum::serve(listener,app.into_make_service()).await.unwrap();
}


async fn hello_world()->Html<&'static str>{
Html("<h1>Hello World from Axum</h1>")
}
async fn handler_404()-> impl IntoResponse{
    (StatusCode::NOT_FOUND,"Invalid Request")
}

async fn get_users(State(pool):State<PgPool>)->impl IntoResponse{
    let users = get_all_users(&pool).await;
    match users {
        Ok(users)=>{
            Json(users).into_response()
        }
        Err(_)=>{
            (StatusCode::INTERNAL_SERVER_ERROR,"Unable to fetch users").into_response()
        }
    }

}

async  fn add_user(State(pool):State<PgPool>,Json(payload):Json<User>)->impl IntoResponse{
    let response = Response{
        message:String::from("User added successfully")
    };
    let result = add_user_to_db(&pool,payload).await;
    match result {
        Ok(_)=>{
            (StatusCode::OK,Json(response)).into_response()
        }
        Err(_)=>{
            (StatusCode::INTERNAL_SERVER_ERROR,"Unable to add user").into_response()
        }
    }
   
}