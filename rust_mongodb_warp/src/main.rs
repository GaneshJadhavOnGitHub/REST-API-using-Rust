/// Entry point of the application.
/// Initialises json log.
/// Checks whether database is up and running.
/// Get routes and starts server.
use crate::db_layer::*;
use warp::{Filter, Rejection};
type Result<T> = std::result::Result<T, error::BoxOfficeError>;
type WebResult<T> = std::result::Result<T, Rejection>;

/// module contains business logic.
/// Function to decide verdict depending upon calculated profit or loss.
mod business_layer;
/// module lists all the constants in the application.
mod constants;
/// module performs database operations.
mod db_layer;
/// module to handle errors.
mod error;
/// module contains request handling functions.
mod handler;
/// module to map mongodb fields to rust structure.
mod model;
/// module lists request and response structures.
mod request_response_structs;
/// module contains functions which return response.
mod response;
/// module to route REST request. It calls appropriate function depending upon request.
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("boxoffice_json_log_configuration.yaml", Default::default()).unwrap();

    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {:?} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    let is_connected_to_db = DB::check_mongodb_is_running().await.unwrap();
    let movie_routes = routes::assets_filter().await;
    let routes = movie_routes.recover(error::handle_rejection).with(log);
    if is_connected_to_db {
        log::info!("Listening on port 8080.");
        println!("Listening on port 8080.");
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    } else {
        log::error!("Can not start server.");
        println!("Can not start server.");
    }
    Ok(())
}
