use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub border: bool,
    pub padding: usize,
}


#[derive(Debug, Deserialize)]
struct ConfigFile {
    config: Config,
}

impl Config {
    // load config, if it doesn't exist, create it
    pub fn load_or_default() -> Self {
        let mut path = dirs::home_dir().expect("Failed to get home directory");
        path.push(".config/evafetch/config.toml");

        if !path.exists() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("Failed to create config directory");
            }
            fs::write(
                &path,
                "[config]\nborder = true\npadding = 2\n",
            )
            .expect("Failed to write default config");
        }

        let content = fs::read_to_string(&path).expect("Failed to read config.toml");

        // deseralize into config struct
        let file: ConfigFile = toml::from_str(&content).unwrap_or(ConfigFile {
            config: Config { border: true, padding: 2 },
        });

        file.config
    }
}
