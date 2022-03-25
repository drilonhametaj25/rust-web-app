#![allow(unused)] // silence unused warnings
use std::env;
use std::sync::Arc;
use crate::model::init_db;
use crate::web::start_web;

mod model; // Model layer
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &'static str = "web/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main(){
    // compute the web_folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = DEFAULT_WEB_PORT;

    // det the database
    let db = init_db().await.expect("Cannot init db");
    let db = Arc::new(db);

    // Start the server
    match start_web(&web_folder, web_port, db).await {
        Ok(_) => println!("SERVER ENDED"),
        Err(ex) => println!("ERROR - web server faild to start")
    }
}