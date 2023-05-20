#[macro_use]
extern crate rocket;

use std::result::Result;

use dotenvy::dotenv;
mod actions;
mod entities;
mod helpers;
mod libs;
mod types;
use actions::todos::TodoAction;
use entities::todo;
use types::BizResp;

use helpers::{failure_ret, success_ret};
use rocket::form::Form;
use rocket::serde::json::Json;
use sea_orm::ActiveValue::{self, NotSet, Set, Unchanged};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

#[derive(FromForm)]
struct TodoCreateForm {
    pub title: String,
    pub content: String,
    pub done: bool,
}

async fn set_up_db(db_url: String) -> Result<DatabaseConnection, DbErr> {
    let db: DatabaseConnection = Database::connect(db_url).await?;
    Ok(db)
}

#[derive(Serialize, Deserialize)]
pub struct GetTodosRespData {
    pub total: u64,
    pub todos: Vec<todo::Model>,
}

#[get("/todos?<page_no>&<page_size>")]
async fn get_todos(
    db: &rocket::State<DatabaseConnection>,
    page_no: u64,
    page_size: u64,
) -> Json<BizResp<GetTodosRespData>> {
    if page_no == 0 {
        return failure_ret("page_no must be greater than 0".to_string(), None);
    }
    let ta = TodoAction::new(db);
    let (todos, total) = ta.get_todos(page_no, page_size).await.unwrap();
    success_ret(GetTodosRespData { todos, total })
}

#[get("/todo?<id>")]
async fn get_todo(db: &rocket::State<DatabaseConnection>, id: i32) -> Json<BizResp<todo::Model>> {
    let ta = TodoAction::new(db);
    ta.get_todo_by_id(id)
        .await
        .map_or_else(|e| failure_ret(e.to_string(), None), success_ret)
}

#[post("/todo/delete?<id>")]
async fn delete_todo(
    db: &rocket::State<DatabaseConnection>,
    id: i32,
) -> Json<BizResp<Option<String>>> {
    let ta = TodoAction::new(db);
    match ta.delete_todo_by_id(id).await {
        Ok(()) => success_ret(None),
        Err(e) => failure_ret(e.to_string(), None),
    }
}

#[post("/todo/update/done?<done>&<id>")]
async fn update_todo_done(
    db: &rocket::State<DatabaseConnection>,
    id: i32,
    done: bool,
) -> Json<BizResp<Option<String>>> {
    let ta: TodoAction = TodoAction::new(db);
    match ta.update_todo_done(id, done).await {
        Ok(()) => success_ret(None),
        Err(e) => failure_ret(e.to_string(), None),
    }
}

#[post("/todo/create", data = "<todo_data>")]
async fn create_todo(
    db: &rocket::State<DatabaseConnection>,
    todo_data: Option<Form<TodoCreateForm>>,
) -> Result<Json<i32>, String> {
    let ta = TodoAction::new(db);
    match todo_data {
        None => return Err("todo is none".to_string()),
        Some(f) => {
            let TodoCreateForm {
                title,
                content,
                done,
            } = f.into_inner();
            match ta
                .create_todo(todo::ActiveModel {
                    title: Set(title),
                    content: Set(content),
                    done: Set(done),
                    ..Default::default()
                })
                .await
            {
                Ok(todo_id) => Ok(Json(todo_id)),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("PG_URL must be set");
    let db = set_up_db(db_url).await.unwrap();
    rocket::build()
        .mount(
            "/",
            routes![get_todos, get_todo, delete_todo, update_todo_done, create_todo],
        )
        .manage(db)
}
