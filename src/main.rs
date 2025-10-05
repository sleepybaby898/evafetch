mod quotes;
mod display;
mod config;

use quotes::Quotes;
use config::Config;
use display::print_quote;

fn main() {
    // load config
    let config = Config::load_or_default();

    // check if quotes exists
    let quotes_path = Quotes::check_file();

    // load quotes
    let quotes = Quotes::load_from_file(&quotes_path);

    // select quote
    let selected = quotes.random_quote();
    print_quote(Some(selected), config.border, config.padding);
}
