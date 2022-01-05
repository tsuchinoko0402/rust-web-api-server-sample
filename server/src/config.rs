use config::ConfigError;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

/// .evnの内容を保存するstruct
#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_address: String,
    pub server_port: i32,
    pub database_url: String,
}

impl Config {
    /// 環境変数からデータを読み込む
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

/// static変数の初期化
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config::from_env().unwrap()
});
