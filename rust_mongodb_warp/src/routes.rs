#![allow(opaque_hidden_inferred_bound)]

use crate::db_layer::*;
use crate::handler;
/// Set endpoints , routes REST requests to handlers.
use std::convert::Infallible;
use warp::Filter;

/// Set endpoints (handlers functions) for REST requests using warp Filter.
pub async fn assets_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db = DB::init().await.unwrap();
    let movie = warp::path("boxoffice")
        .and(warp::path("movies"))
        .and(warp::path("v1"));
    let movie_routes = movie
        .and(warp::post())
        .and(warp::path("createmovie"))
        .and(warp::path::end())
        .and(warp::body::aggregate())
        .and(with_db(db.clone()))
        .and_then(handler::create_movie_handler)
        .or(movie
            .and(warp::get())
            .and(warp::path("readmovies"))
            .and(warp::path::end())
            .and(with_db(db.clone()))
            .and_then(handler::read_movies_handler))
        .or(movie
            .and(warp::put())
            .and(warp::path("updatemovie"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(warp::body::aggregate())
            .and(with_db(db.clone()))
            .and_then(handler::update_movie_handler))
        .or(movie
            .and(warp::delete())
            .and(warp::path("deletemovie"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(with_db(db.clone()))
            .and_then(handler::delete_movie_handler));
    return movie_routes.boxed();
}

/** Database operations are performed through each handler functions.
so while setting end-points i.e. while routing
we are passing DB object containing MONGODB Client to each handler.
A MONGODB Client is used to connect to MONGODB.
This function is called while setting end-points using warp Filter. */
fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
