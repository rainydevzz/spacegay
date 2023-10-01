use dotenv::dotenv;
use sea_orm::{DatabaseConnection, Database, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use std::env;
use axum::{Router, response::{Html, Response}, routing::{get, post}, Extension, http::{Request, StatusCode}, middleware::{Next, self}};
use serde::{Deserialize, Serialize};

use crate::entity::users;

mod entity;

#[derive(Deserialize, Debug, Serialize)]
struct Login {
    email: String,
    password: String
}

async fn midd<B> (request: Request<B>, next: Next<B>) -> Response {
    println!("{:?}", &request.headers());
    
}

async fn index() -> Html<String> {
    Html(std::fs::read_to_string("src/static/index.html").unwrap())
}

async fn index_login(Extension(conn): Extension<DatabaseConnection>, axum::Form(payload): axum::Form<Login>) -> String {
    let res: Option<users::Model> = users::Entity::find()
        .filter(users::Column::Name.contains(&payload.email))
        .filter(users::Column::Password.contains(&payload.password))
        .one(&conn)
        .await
        .unwrap();
    if res.is_none() {
        let data = users::ActiveModel {
            name: Set(payload.email),
            password: Set(payload.password),
            token: Set("123".to_owned())
        };

        let data: users::Model = data.insert(&conn).await.unwrap();
        println!("{data:?}");
    } else {
        println!("{res:?}");
    }
    "a".to_owned()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url: &String = &env::vars().find(|v: &(String, String)| v.0 == "DATABASE_URL").unwrap().1;

    let conn: DatabaseConnection = Database::connect(url).await.unwrap();

    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(
            Router::new()
                .route("/", get(index))
                .route("/", post(index_login))
                .layer(Extension(conn))
                .layer(middleware::from_fn(midd))
                .into_make_service()
        )
        .await
        .unwrap();
}