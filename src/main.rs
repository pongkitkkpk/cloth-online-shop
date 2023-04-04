use actix_web::{middleware, App, HttpServer};
mod handlers;
mod models;
mod routes;
use crate::routes::{
    admin_cloth_route, admin_collection_route, admin_homepage_route, user_basket_route,
    user_cloth_route, user_collection_route,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .configure(admin_collection_route::config)
            // .configure(deleteforadmin_route::config)
            .configure(admin_homepage_route::config)
            .configure(admin_cloth_route::config)
            .configure(user_collection_route::config)
            .configure(user_cloth_route::config)
            .configure(user_basket_route::config)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8082))?
    .run();

    println!("Server running...");
    server.await
}
