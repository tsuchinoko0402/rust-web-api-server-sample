mod application;
mod config;
mod domain;
mod infra;
mod auth;

use actix_web::{dev::ServiceRequest, Error};
use auth::validate_token;
use simplelog::*;
use std::fs::File;
use anyhow::Result;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

#[macro_use]
extern crate diesel;
extern crate log;
extern crate simplelog;

fn init_logger() {
    simplelog::CombinedLogger::init(vec![
        // 標準出力には Debug 以上を表示する。
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // ファイル server.log には info 以上を表示する。
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("server.log").unwrap(),
        ),
    ])
    .unwrap();
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    init_logger();
    infra::actix::router::run()
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}
