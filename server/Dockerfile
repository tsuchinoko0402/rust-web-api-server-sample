# 開発環境
FROM rust:1.57.0 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
RUN apt install -y libpq-dev
RUN cargo install diesel_cli
COPY . .
# ビルド環境
FROM develop-stage as build-stage
RUN update-ca-certificates
RUN cargo build --release
# 本番環境
FROM rust:1.57.0-slim-buster as production-stage
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=build-stage /app/target/release/actix_web_sample .
EXPOSE 8088
CMD ["/usr/local/bin/api"]
