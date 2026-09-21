mod error;

use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

pub use error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub handle_name: String,
    pub hostname: String,
    pub public_key: String,
    pub private_key: String,
}

fn load_config(path: &str) -> Result<Config, Error> {
    let text = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&text)?;
    Ok(config)
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| load_config("src/config.json").expect("Can't load!"))
}
