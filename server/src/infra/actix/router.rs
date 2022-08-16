use super::handlers;
use crate::{config::CONFIG, domain::models::pokemon::pokemon_repository::PokemonRepository};
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT")
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(CONFIG.server_port);

    HttpServer::new(|| {
        App::new()
            .app_data(RequestContext::new())
            .wrap(Logger::default())
            .service(handlers::health)
            .service(handlers::post_pokemon)
            .service(handlers::get_pokemon)
            .service(handlers::update_pokemon)
            .service(handlers::delete_pokemon)
            .service(handlers::get_pokemon_list)
    })
    .bind(format!("{}:{}", CONFIG.server_address, port))?
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
