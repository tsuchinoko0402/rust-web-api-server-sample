use super::router::RequestContext;
use crate::application::pokemon_application::PokemonApplicationService;
use crate::application::pokemon_data::PokemonData;
use crate::infra::actix::request::PokemonRequest;
use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};

#[post("/pokemon")]
async fn post_pokemon(
    data: web::Data<RequestContext>,
    request: Json<PokemonRequest>,
) -> impl Responder {
    let pokemon_application = PokemonApplicationService::new(data.pokemon_repository());
    match pokemon_application.register(PokemonData::new(request.of())) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
