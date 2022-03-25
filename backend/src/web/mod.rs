use std::sync::Arc;
use crate::model::Db;
use std::path::Path;
use warp::Filter;


pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
    }

    // Static content
    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    
    let static_site = content.or(root_index);

    let routes = static_site;

    println!("Start server");
    warp::serve(routes).run(([127,0,0,1], web_port)).await;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start")]
    FailStartWebFolderNotFound(String)
}