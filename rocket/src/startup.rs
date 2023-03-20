use rocket::{Config, Rocket, Build, figment::providers::{Format, Toml}, log::LogLevel};

/// 无外部配置文件
#[allow(dead_code)]
pub fn start_with_default_config() -> Rocket<Build> {
    let mut config = Config::default();
    config.port = 80;
    config.log_level = LogLevel::Normal;
    let app = rocket::custom(config);
    app
}

/// 通过figment读取配置，默认读取Rocket.toml文件
#[allow(dead_code)]
pub fn start_with_figment() -> Rocket<Build> {
    let config = Config::figment();
    let toml = Toml::file("MyRocket.toml").nested();
    let config = config.merge(toml);
    let app = rocket::custom(config);
    app
}