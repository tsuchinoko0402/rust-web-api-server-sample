#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use thiserror::Error;

// customerテーブルの日付型カラム(reg_date)を扱うのに必要
use chrono::NaiveDate;

// 環境変数の読み込み
mod config;
// "src/schema.rs"で定義したマクロを使えるようにする
mod schema;
use schema::customer::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// テーブルcustomerの各行の情報を格納する構造体
// （Web上のサンプルプロブラムでは別ソースmodels.rsに書かれていることが多い）
// ageはNULLを取ることがあり、Option型で受ける必要あり
#[derive(Queryable, Debug)]
struct Customer {
    name: String,
    age: Option<i32>,
    reg_date: NaiveDate,
}

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    // customerテーブルの各情報を取得
    // SQLで「SELECT * FROM customer;」をやっているのと同じ
    let results = customer
        .load::<Customer>(&conn)
        .expect("Error loading customer");

    // 結果を表示
    for r in &results {
        println!("{:?}", r);
    }

    let response_body = format!("Hello {}! age:{:?}", results[0].name, results[0].age);
    HttpResponse::Ok().body(response_body)
}

#[get("/{id}/{name_1}/index.html")]
async fn display_id_and_name(web::Path((id, name_1)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name_1, id)
}

/**
 * メインメソッド：Web サーバーを起動する
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server_address = format!("{}:{}", config.server_address, config.server_port);
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .wrap(Logger::default())
            .data(pool.clone())
    })
    .bind(server_address)?
    .run()
    .await?;
    Ok(())
}
