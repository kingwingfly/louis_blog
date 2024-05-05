use actix_web::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};

fn app() -> impl IntoView {
    use app::*;
    view! { <App />}
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(app);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;

        App::new()
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), app)
            .wrap(middleware::Compress::default())
    })
    .bind(addr)?
    .run()
    .await
}