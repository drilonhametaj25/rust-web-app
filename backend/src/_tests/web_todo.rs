

#[tokio::test]
async fn web_todo_list() -> Result<()> {

    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone());

    let resp = warp::test:request().method("get").path("/api/todos").reply(&todo_api).await;

    assert_eq!(200, resp.status(), "http status");
    
    Ok(())
}