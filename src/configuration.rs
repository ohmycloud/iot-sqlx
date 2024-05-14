// The possible runtime environment
enum Environment {
    Local,
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

// Try to convert from String to Environment
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use `local`, `development` or `production`.",
                other
            )),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub mysql: MysqlSettings,
}


#[derive(serde::Deserialize)]
pub struct MysqlSettings {
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub database: String,
    pub max_connections: u32,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment
    // Default to `local` if not specified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;
    // Try to convert the configuration values it read into Settings type.
    settings.try_deserialize::<Settings>()
}