use config::ConfigError;
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
