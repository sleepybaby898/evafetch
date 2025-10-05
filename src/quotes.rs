use rand::Rng;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub speaker: String,
}

#[derive(Debug, Deserialize)]
struct QuotesFile {
    quote: Vec<Quote>,
}

pub struct Quotes {
    quotes: Vec<Quote>,
}

// embed default quotes into binary
const DEFAULT_QUOTES: &str = include_str!("quotes.toml");

impl Quotes {
    // check if file exists, if it doesn't create with default quotes
    pub fn check_file() -> String {
        let mut path = dirs::home_dir().expect("Failed to get home directory");
        path.push(".config/evafetch/quotes.toml");

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Failed to create config directory");
            }
            fs::write(&path, DEFAULT_QUOTES).expect("Failed to write default quotes file");
            println!("Created default quotes.toml at {}", path.display());
        }

        path.to_str().unwrap().to_string()
    }

    // load
    pub fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read quotes file");
        let quotes_file: QuotesFile =
            toml::from_str(&content).expect("Failed to parse quotes.toml");

        Quotes {
            quotes: quotes_file.quote,
        }
    }

    // get quote
    pub fn random_quote(&self) -> &Quote {
        let mut rng = rand::thread_rng();
        &self.quotes[rng.gen_range(0..self.quotes.len())]
    }
}
