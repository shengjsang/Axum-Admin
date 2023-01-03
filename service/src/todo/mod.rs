use anyhow::Result;
use model::entity::todo;
use todo::Entity as Todo;

pub async fn create_task() -> Result<String> {
    Ok("创建成功".to_string())
}
