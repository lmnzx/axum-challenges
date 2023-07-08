use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name=lemon
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

// /hello/lemon
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello, {name}!</h1>", name = name))
}

// router for /hello and /hello2
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// serve static files from ./src
fn router_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    // println!("->> {:12} - main_response_mapper - {res:?}", "RES_MAPPER");

    println!();
    res
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = model::ModelController::new().await?;

    let routes_api = web::routes_tickets::router(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(router_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("->> LISTENING ON: {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
