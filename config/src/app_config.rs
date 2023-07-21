pub struct AppConfig {
    pub db_protocol: String,
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String,
    pub jwt_secret: String,
    pub db_url_origin: String,
    pub db_url: String,
    pub host: String,
    pub port_websocket: String,
    pub port_web: String,
    pub broker_url: String,
}


// You can specify a value at build time, or you can enter it directly here
impl Default for AppConfig {
    fn default() -> Self {
        let mut db_url_origin = std::env::var("DB_PROTOCOL").unwrap_or("mysql".to_string());
        db_url_origin.push_str("://");
        db_url_origin.push_str(&std::env::var("DB_USERNAME").unwrap());
        db_url_origin.push_str(":");
        db_url_origin.push_str(&std::env::var("DB_PASSWORD").unwrap().to_string());
        db_url_origin.push_str("@");
        db_url_origin.push_str(&std::env::var("DB_HOST").unwrap().to_string());
        db_url_origin.push_str(":");
        db_url_origin.push_str(&std::env::var("DB_PORT").unwrap_or("3306".to_string()));
        let mut db_url = db_url_origin.clone();
        db_url.push_str("/");
        db_url.push_str(&std::env::var("DB_DATABASE").unwrap().to_string());
        Self {
            db_protocol: std::env::var("DB_PROTOCOL").unwrap_or("mysql".to_string()),
            db_host: std::env::var("DB_HOST").unwrap(),
            db_port: std::env::var("DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap(),
            db_password: std::env::var("DB_PASSWORD").unwrap(),
            db_database: std::env::var("DB_DATABASE").unwrap(),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("Please set the JWT_SECRET env variable."),
            db_url_origin: db_url_origin,
            db_url: db_url,
            host: std::env::var("HOST").unwrap(),
            port_websocket: std::env::var("PORT_WEBSOCKET").unwrap_or("8080".to_string()),
            port_web: std::env::var("PORT_WEB").unwrap_or("8000".to_string()),
            broker_url: std::env::var("BROKER_URL").unwrap(),
        }
    }
}