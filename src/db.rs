use serde::{Deserialize, Serialize};
use std::error::Error;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug,Hash)]
pub struct User {
    name : String,
    surname:String,
    city:String,
    state:String,
    country:String,
}



pub async fn get_all_users(pool:&sqlx::PgPool)->Result<Vec<User>,Box<dyn Error>>{
    let q = "SELECT * FROM user_information";
   let query = sqlx::query(q);
    let rows = query.fetch_all(pool).await?;
    let data:Vec<User> = rows.iter().map(|row|User{
        name: row.get("name"),
        surname: row.get("surname"),
        city: row.get("city"),
        state: row.get("state"),
        country: row.get("country"),

    }).collect();
    Ok(data)
}

pub async  fn add_user_to_db(pool:&sqlx::PgPool,user:User)->Result<(),Box<dyn Error>>{
    let query = "INSERT INTO user_information (name,surname,city,state,country) VALUES ($1,$2,$3,$4,$5)";
    sqlx::query(&query)
    .bind(&user.name) 
    .bind(&user.surname) 
    .bind(&user.city) 
    .bind(&user.state) 
    .bind(&user.country)
    .execute(pool)
    .await?;
    Ok(())
}