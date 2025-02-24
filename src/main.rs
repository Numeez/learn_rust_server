use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
   let app = Router::new().route("/",get(hello_world));
   let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
   .await
   .unwrap();
   println!("listening on {}", listener.local_addr().unwrap());
   axum::serve(listener,app).await.unwrap();
}


async fn hello_world()->Html<&'static str>{
Html("<h1>Hello World from Axum</h1>")
}