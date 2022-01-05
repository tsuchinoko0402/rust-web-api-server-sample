mod application;
mod config;
mod domain;
mod infra;

#[macro_use]
extern crate diesel;

fn main() -> std::io::Result<()> {
    infra::actix::router::run()
}
