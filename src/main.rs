mod controllers;
mod services;

use std::env;
use actix::prelude::*;
use actix_web::{App, HttpServer};
use actix_files::Files;
use crate::controllers::site_handler;
use crate::services::storage::Storage;
use crate::services::scheduler::Scheduler;
use crate::services::scheduler::HttpsCertTask;

fn main() {
    //create actix runtime
    let sys = System::new("app");

    let storage = Storage::new("./storage.db");

    let cert_checker = HttpsCertTask::new(storage.clone());

    //init web server
    let port = env::var("PORT").unwrap_or(String::from("8088"));

    HttpServer::new(move || {
        App::new()
        .data(storage.clone())
        .service(site_handler::list_sites)
        .service(site_handler::add_site)
        .service(site_handler::update_site)
        .service(site_handler::remove_site)
        .service(Files::new("/", "./web/build").index_file("index.html"))
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .start();


    //init scheduler
    let mut scheduler = Scheduler::new();
    scheduler.add_task("0 0 * * * * *", Box::new(cert_checker)).unwrap();
    let _ = scheduler.start();

    //start actix runtime
    let _ = sys.run();
}
