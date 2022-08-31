use dotenv::dotenv;

#[derive(Debug)]
pub struct MongoConfig {
    pub mongo_uri: String,
    pub db_name: String,
}

#[derive(Debug)]
pub struct Config {
    pub mongo: MongoConfig,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI is not set in env!");

        let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in env!");

        Config {
            mongo: MongoConfig { mongo_uri, db_name },
        }
    }
}
