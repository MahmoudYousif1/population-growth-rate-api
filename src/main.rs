pub mod app_state;
pub mod handlers;
pub mod operations;
pub mod utils;
pub mod validations;

use crate::app_state::country_loader::load_country_mappings;
use crate::app_state::persistence::spawn_persistence_tasks;
use crate::utils::config::load;
use crate::utils::routes::configure_routes;
use actix_web::{App, HttpServer, dev::ServerHandle, web};
use dotenv::dotenv;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let cfg = load();
    let mut app_state = load_country_mappings()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    app_state.mappings_file_path = cfg.country_population_file_path.clone();

    let shared_state = web::Data::new(app_state);

    let bind_addr = format!("{}:{}", cfg.host, cfg.port);
    println!("Server running at http://{}/", bind_addr);

    let tasks_state = shared_state.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .configure(configure_routes)
    })
    .workers(cfg.workers)
    .bind(&bind_addr)?
    .run();

    let handle: ServerHandle = server.handle();
    spawn_persistence_tasks(tasks_state, cfg.save_interval_minutes, handle);

    server.await
}
