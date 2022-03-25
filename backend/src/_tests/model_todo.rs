use crate::model::db::init_db;
use super::TodoMac;
use crate::model::todo::TodoPatch;
use crate::security::utx_from_token;

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {

    let db = init_db().await?;
    let utx = utx_from_token("123").await?;
    let todos = TodoMac::list(&db, &utx).await?;

    assert_eq!(2, todos.len(), "todos list length");
    
    Ok(())
}

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {

    let db = init_db().await?;

    let data_fx = TodoPatch {
        title: Some("Test- model todo create".to_string()),
        ..Default::default()
    }; 
    let utx = utx_from_token("123").await?;

    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    
    Ok(())
}