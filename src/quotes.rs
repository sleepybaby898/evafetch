use rand::Rng;
use serde::Deserialize;

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

impl Quotes {

    pub fn load_from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path).expect("Failed to read quotes file");
        let file: QuotesFile = toml::from_str(&content).expect("Failed to parse quotes.toml");
        Quotes { quotes: file.quote }
    }


    pub fn random_quote_indexed(&self) -> (&Quote, usize) {
        if self.quotes.is_empty() {
            panic!("No quotes found");
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.quotes.len());
        (&self.quotes[idx], idx + 1)
    }

    pub fn total(&self) -> usize {
        self.quotes.len()
    }
}
