use anyhow::{anyhow, Result};
use chrono::Local;
use model::entity::todo;
use model::entity::todo::Model;
use model::todo::request::CreateReq;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait};
use todo::Entity as Todo;

pub async fn create_task(db: &DatabaseConnection, req: CreateReq) -> Result<String> {
    let task = todo::ActiveModel {
        title: Set(Some(req.title)),
        content: Set(Some(req.content)),
        status: Set(Some(false)),
        created_at: Set(Some(Local::now().naive_local())),
        updated_at: Set(None),
        id: Default::default(),
    };

    Todo::insert(task).exec(db).await?;

    Ok("创建成功".to_string())
}

pub async fn change_status(db: &DatabaseConnection, id: i32) -> Result<Model> {
    let res = Todo::find_by_id(id).one(db).await?;
    let mut todo: todo::ActiveModel = match res {
        None => {
            return Err(anyhow!("未查到对应任务"));
        }
        Some(todo) => todo.into(),
    };

    let status = todo.status.unwrap().unwrap();

    todo.status = Set(Some(!status));

    let todo: Model = todo.update(db).await?;

    Ok(todo)
}

pub async fn delete_task(db: &DatabaseConnection, id: i32) -> Result<String> {
    let _res: DeleteResult = Todo::delete_by_id(id).exec(db).await?;
    Ok("删除成功".to_string())
}

pub async fn delete_all_finish_task(db: &DatabaseConnection) -> Result<String> {
    todo!();
}

pub async fn get_all_tasks(db: &DatabaseConnection) -> Result<Vec<Model>> {
    let tasks = Todo::find().all(db).await?;
    Ok(tasks)
}

pub async fn get_task_by_id(db: &DatabaseConnection, id: i32) -> Result<Model> {

    let res = Todo::find_by_id(id).one(db).await?;
    let todo = match res {
        None => {
            return Err(anyhow!("未查到对应任务"));
        }
        Some(todo) => todo
    };

    Ok(todo)
}
