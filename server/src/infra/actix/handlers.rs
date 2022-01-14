use super::router::RequestContext;
use crate::application::pokemon_delete_service::PokemonDeleteService;
use crate::application::pokemon_get_service::PokemonGetService;
use crate::application::pokemon_list_service::PokemonListService;
use crate::application::pokemon_update_service::{PokemonUpdateCommand, PokemonUpdateService};
use crate::application::{
    pokemon_data::PokemonData, pokemon_register_service::PokemonRegisterService,
};
use crate::infra::actix::request::PokemonRequest;
use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    r#type: String,
}

#[post("/pokemon")]
async fn post_pokemon(
    data: web::Data<RequestContext>,
    request: Json<PokemonRequest>,
) -> impl Responder {
    let pokemon_application = PokemonRegisterService::new(data.pokemon_repository());
    let data = PokemonData::new(request.of());
    match pokemon_application.handle(data.clone()) {
        Ok(_) => HttpResponse::Ok().body("SUCCESS Register Pokemon"),
        Err(_) => {
            let response = ErrorResponse {
                message: format!("FAILURE Register Pokemon: {:?}", data.clone()),
                r#type: "get_pokemon_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/pokemon/{number}")]
async fn get_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonGetService::new(data.pokemon_repository());
    let no = path_params.into_inner().0.into();
    match pokemon_application.handle(no) {
        Ok(pokemon) => HttpResponse::Ok().json(pokemon),
        Err(_) => {
            let response = ErrorResponse {
                message: format!("FAILURE Get Pokemon: no {:?}", no),
                r#type: "get_pokemon_list_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/pokemon")]
async fn get_pokemon_list(data: web::Data<RequestContext>) -> impl Responder {
    let pokemon_application = PokemonListService::new(data.pokemon_repository());
    match pokemon_application.handle() {
        Ok(pokemon) => HttpResponse::Ok().json(pokemon),
        Err(_) => {
            let response = ErrorResponse {
                message: "FAILURE Get Pokemon List".to_string(),
                r#type: "get_pokemon_list_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[put("/pokemon/{number}")]
async fn update_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
    request: Json<PokemonRequest>,
) -> impl Responder {
    let pokemon_application = PokemonUpdateService::new(data.pokemon_repository());
    let no = path_params.into_inner().0.into();
    let mut update_command = PokemonUpdateCommand::new(no);
    update_command.set_name(Some(request.of().name.into()));
    update_command.set_types(Some(request.of().types.into()));
    match pokemon_application.handle(update_command) {
        Ok(_) => HttpResponse::Ok().body(format!("SUCCESS Update Pokemon: no {}", no)),
        Err(_) => {
            let response = ErrorResponse {
                message: format!("FAILURE Update Pokemon: no {}", no),
                r#type: "update_pokemon_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[delete("/pokemon/{number}")]
async fn delete_pokemon(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let pokemon_application = PokemonDeleteService::new(data.pokemon_repository());
    let no = path_params.into_inner().0.into();
    match pokemon_application.handle(no) {
        Ok(_) => HttpResponse::Ok().body(format!("SUCCESS Delete Pokemon: no {}", no)),
        Err(_) => {
            let response = ErrorResponse {
                message: format!("FAILURE Delete Pokemon: no {}", no),
                r#type: "delete_pokemon_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
