use super::router::RequestContext;
use crate::application::pokemon_delete_service::PokemonDeleteService;
use crate::application::pokemon_get_service::PokemonGetService;
use crate::application::pokemon_list_service::PokemonListService;
use crate::application::pokemon_update_service::{PokemonUpdateService, PokemonUpdateCommand};
use crate::application::{pokemon_data::PokemonData, pokemon_register_service::PokemonRegisterService};
use crate::infra::actix::request::PokemonRequest;
use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};

#[post("/pokemon")]
async fn post_pokemon(
    data: web::Data<RequestContext>,
    request: Json<PokemonRequest>,
) -> impl Responder {
    let pokemon_application = PokemonRegisterService::new(data.pokemon_repository());
    match pokemon_application.handle(PokemonData::new(request.of())) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/pokemon/{number}")]
async fn get_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonGetService::new(data.pokemon_repository());
    match pokemon_application.handle(path_params.into_inner().0.into()) {
        Ok(pokemon) => HttpResponse::Ok().json(pokemon),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/pokemon")]
async fn get_pokemon_list(data: web::Data<RequestContext>) -> impl Responder {
    let pokemon_application = PokemonListService::new(data.pokemon_repository());
    match pokemon_application.handle() {
        Ok(pokemon) => HttpResponse::Ok().json(pokemon),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[put("/pokemon/{number}")]
async fn update_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
    request: Json<PokemonRequest>,
) -> impl Responder {
    let pokemon_application = PokemonUpdateService::new(data.pokemon_repository());
    let mut update_command = PokemonUpdateCommand::new(path_params.into_inner().0.into());
    update_command.set_name(Some(request.of().name.into()));
    update_command.set_types(Some(request.of().types.into()));
    match pokemon_application.handle(update_command) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[delete("/pokemon/{number}")]
async fn delete_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonDeleteService::new(data.pokemon_repository());
    match pokemon_application.handle(path_params.into_inner().0.into()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
