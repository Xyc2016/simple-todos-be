use crate::entities::todo::{ActiveModel, Entity as Todo, Model};
use crate::libs::db::limit_offset;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QuerySelect, ActiveValue::{self, Set, NotSet}};

pub struct TodoAction<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> TodoAction<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        TodoAction { db }
    }

    pub async fn get_todos(&self, page_no: u64, page_size: u64) -> Result<Vec<Model>, DbErr> {
        let (limit, offset) = limit_offset(page_no, page_size);
        dbg!(limit, offset);
        let todos = Todo::find()
            .limit(limit)
            .offset(offset)
            .all(self.db)
            .await;
        todos
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Result<Model, DbErr> {
        match Todo::find_by_id(id).one(self.db).await {
            Ok(todo) => {
                match todo {
                    Some(t) => Ok(t),
                    None => return Err(DbErr::RecordNotFound(format!("Todo with id {} not found", id))),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Result<(), DbErr> {
        match Todo::delete_by_id(id).exec(self.db).await {
            Ok(todo) => {
                return Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub async fn create_todo(
        &self,
        todo: Model,
    ) -> Result<i32, DbErr> {
        let todo: ActiveModel = todo.into();
        let a = Todo::insert(todo).exec(self.db).await?;
        Ok(a.last_insert_id)
    }

}
