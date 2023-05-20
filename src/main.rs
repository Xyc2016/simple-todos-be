#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
mod actions;
mod entities;
mod helpers;
mod libs;
mod types;
use types::BizResp;
use actions::todos::{self, TodoAction};
use entities::todo;


use helpers::{failure_ret, success_ret};
use rocket::{serde::json::Json, response::status};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde_json::{json, Value};

async fn set_up_db(db_url: String) -> Result<DatabaseConnection, DbErr> {
    let db: DatabaseConnection = Database::connect(db_url).await?;

    Ok(db)
}

#[get("/todos?<page_no>&<page_size>")]
async fn get_todos(
    db: &rocket::State<DatabaseConnection>,
    page_no: u64,
    page_size: u64,
) -> Json<BizResp<Vec<todo::Model>>> {
    if page_no == 0 {
        return failure_ret("page_no must be greater than 0".to_string(), None);
    }
    let ta = TodoAction::new(db);
    let todos = ta.get_todos(page_no, page_size).await.unwrap();
    success_ret(todos)
}

#[get("/todo?<id>")]
async fn get_todo(db: &rocket::State<DatabaseConnection>, id: i32) -> Json<BizResp<todo::Model>> {
    let ta = TodoAction::new(db);
    ta.get_todo_by_id(id).await.map_or_else(
        |e| failure_ret(e.to_string(), None),
        success_ret,
    )
}

#[post("/todo/delete?<id>")]
async fn delete_todo(db: &rocket::State<DatabaseConnection>, id: i32) -> Json<BizResp<Option<String>>> {
    let ta = TodoAction::new(db);
    match ta.delete_todo_by_id(id).await {
        Ok(()) => success_ret(None),
        Err(e) => failure_ret(e.to_string(), None),
    }
}

// #[post("/todo/create?<id>")]
// async fn create_todo(
//     db: &rocket::State<DatabaseConnection>,
//     id: i32,
// ) -> Result<Json<todo::Model>, String> {
//     let ta = TodoAction::new(db);
//     match ta.get_todo_by_id(id).await {
//         Ok(todo) => Ok(Json(todo)),
//         Err(e) => Err(e.to_string()),
//     }
// }

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("PG_URL must be set");
    let db = set_up_db(db_url).await.unwrap();
    rocket::build()
        .mount("/", routes![get_todos, get_todo, delete_todo])
        .manage(db)
}
