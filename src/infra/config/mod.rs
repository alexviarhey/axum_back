#[derive(Debug)]
pub struct Config {
    pub mongo: MongoConfig,
    pub service: ServiceConfig,
}

impl Config {
    pub fn earew() -> Self {
        let app_mode = std::env::var("APP_MODE").unwrap_or_else(|_| "development".to_string());

        dotenv::from_filename(format!(".env.{}", app_mode)).ok();

        let mongo_config = MongoConfig::new();
        let service_config = ServiceConfig::new();

        Config {
            mongo: mongo_config,
            service: service_config,
        }
    }
}

#[derive(Debug)]
pub struct MongoConfig {
    pub mongo_uri: String,
    pub db_name: String,
}

impl MongoConfig {
    fn new() -> Self {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI is not set in env!");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in env!");

        Self { mongo_uri, db_name }
    }
}

#[derive(Debug)]
pub struct ServiceConfig {
    pub address: String,
    pub port: String,
}

impl ServiceConfig {
    fn new() -> Self {
        let address = std::env::var("ADDRESS").expect("ADDRESS is not set in env!");
        let port = std::env::var("PORT").expect("PORT is not set in env!");

        Self { address, port }
    }
}
