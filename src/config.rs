#[derive(Clone)]
pub struct AppConfig {
    /// Database connection string, typically fetched from `.env`.
    pub database_url: String,
    /// Address that Actix should bind to, defaults to `127.0.0.1:8080`.
    pub bind_address: String,
    /// Secret used to sign JWTs.
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for the application");
        let bind_address =
            std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change-me".to_string());

        Self {
            database_url,
            bind_address,
            jwt_secret,
        }
    }
}
