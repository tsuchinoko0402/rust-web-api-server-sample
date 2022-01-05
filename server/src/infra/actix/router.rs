use super::handlers;
use crate::config::CONFIG;
use actix_web::{App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::domain::services::pokemon_repository::PokemonRepository;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .service(handlers::hello)
            .service(handlers::post_pokemon)
            .service(handlers::get_pokemon) 
            .service(handlers::update_pokemon) 
    })
    .bind(format!("{}:{}", CONFIG.server_address, CONFIG.server_port))?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn pokemon_repository(&self) -> impl PokemonRepository {
        use crate::infra::diesel::pokemon_repository::PokemonRepositoryImpl;

        PokemonRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
