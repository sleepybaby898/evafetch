mod quotes;
mod display;
mod config;

use quotes::Quotes;
use config::Config;
use display::print_quote;

fn main() {
    // load config
    let config = Config::load_or_default();

    // load quotes
    let mut quotes_path = dirs::home_dir().expect("Failed to get home directory");
    quotes_path.push(".config/evafetch/quotes.toml");
    let quotes = Quotes::load_from_file(quotes_path.to_str().unwrap());

    // pick a random quote
    let (selected, idx) = quotes.random_quote_indexed();

    print_quote(
        Some(selected),
        config.border,
        config.padding,
        if config.numbering { Some(idx) } else { None },
        quotes.total(),
    );
}
