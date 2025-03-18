mod auth;
mod config;
mod context;
mod db;
mod web;

use app::*;
use auth::mw_context;
use axum::{middleware::from_fn_with_state, Router};
use config::Config;
use db::Db;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() {
    let filter = filter::Targets::new().with_target(
        "backend",
        if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::INFO
        },
    );
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .without_time()
                .with_target(false),
        )
        .with(filter)
        .init();

    let conf = get_configuration(None).unwrap();
    let config = Config::from_env();
    let db = Db::new(&config.db_url).await.unwrap();

    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .merge(auth::routes(config.clone(), db.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(from_fn_with_state((config, db), mw_context)),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
