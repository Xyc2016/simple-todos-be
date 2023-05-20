use crate::entities::todo::{ActiveModel, Column, Entity as Todo, Model};
use crate::libs::db::limit_offset;

use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, QuerySelect};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};

pub struct TodoAction<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> TodoAction<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        TodoAction { db }
    }

    pub async fn get_todos(
        &self,
        page_no: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let (limit, offset) = limit_offset(page_no, page_size);
        dbg!(limit, offset);
        let todos = Todo::find()
            .limit(limit)
            .offset(offset)
            .all(self.db)
            .await?;
        let count = Todo::find().count(self.db).await?;
        Ok((todos, count))
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Result<Model, DbErr> {
        match Todo::find_by_id(id).one(self.db).await {
            Ok(todo) => match todo {
                Some(t) => Ok(t),
                None => {
                    return Err(DbErr::RecordNotFound(format!(
                        "Todo with id {} not found",
                        id
                    )))
                }
            },
            Err(e) => Err(e),
        }
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Result<(), DbErr> {
        match Todo::delete_by_id(id).exec(self.db).await {
            Ok(dr) => match dr.rows_affected {
                0 => {
                    return Err(DbErr::RecordNotFound(format!(
                        "Todo with id {} not found",
                        id
                    )))
                }
                _ => Ok(()),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update_todo_done(&self, id: i32, done: bool) -> Result<(), DbErr> {
        dbg!(id, done);
        let update_fields: ActiveModel = ActiveModel {
            // id: Set(id),
            done: Set(done),
            ..Default::default()
        };
        Todo::update_many()
            .set(update_fields)
            .filter(Column::Id.eq(id))
            .exec(self.db)
            .await?;
        Ok(())
    }

    pub async fn create_todo(&self, todo: ActiveModel) -> Result<i32, DbErr> {
        let a = Todo::insert(todo).exec(self.db).await?;
        Ok(a.last_insert_id)
    }
}
