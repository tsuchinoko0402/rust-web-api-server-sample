mod application;
mod config;
mod domain;
mod infra;

use simplelog::*;
use std::fs::File;

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
