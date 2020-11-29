#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub db: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub name: String,
    pub password: String,
    pub port: u16,
    pub user: String,
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_app_settings() -> Result<AppSettings, config::ConfigError> {
    let mut settings = config::Config::default();

    // look for any top-level file with an extension that `config` knows how to parse: yaml, json, ...
    settings.merge(config::File::with_name("configuration"))?;

    // try to convert the file's content into our `AppSettings` type
    settings.try_into()
}
