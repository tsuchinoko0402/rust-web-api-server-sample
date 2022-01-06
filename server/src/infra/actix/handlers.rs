use super::router::RequestContext;
use crate::application::pokemon_application::{
    PokemonApplicationService, PokemonDeleteCommand, PokemonUpdateCommand,
};
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

#[get("/pokemon/{number}")]
async fn get_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonApplicationService::new(data.pokemon_repository());
    match pokemon_application.get(path_params.into_inner().0.into()) {
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
    let pokemon_application = PokemonApplicationService::new(data.pokemon_repository());
    let mut update_command = PokemonUpdateCommand::new(path_params.into_inner().0.into());
    update_command.set_name(Some(request.of().name.into()));
    update_command.set_types(Some(request.of().types.into()));
    match pokemon_application.update(update_command) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[delete("/pokemon/{number}")]
async fn delete_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonApplicationService::new(data.pokemon_repository());
    let delete_command = PokemonDeleteCommand::new(path_params.into_inner().0.into());
    match pokemon_application.delete(delete_command) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
