use crate::quotes::Quote;
use unicode_width::UnicodeWidthStr;
use crossterm::terminal::size;

// lightning bolts shoot from my fingertips

pub fn print_quote(
    quote: Option<&Quote>,
    add_border: bool,
    padding: usize,
    quote_index: Option<usize>,
    total_quotes: usize,
) {
    let q = match quote {
        Some(q) => q,
        None => {
            println!("No quote found.");
            return;
        }
    };

    let number_text = if let Some(i) = quote_index {
        format!(" (Quote {} of {})", i, total_quotes)
    } else {
        "".to_string()
    };

    let full_quote = format!("{}{}", q.quote, number_text);

    let speaker_line = format!("— {}", q.speaker);

    // terminal width fuckk
    let term_width = size().map(|(w, _)| w as usize).unwrap_or(80);
    let max_box_width = term_width.saturating_sub(2);

    let wrap_width = max_box_width.saturating_sub(2 * padding);
    let mut wrapped_lines = wrap_text(&full_quote, wrap_width);

    if let Some(first) = wrapped_lines.first_mut() {
        *first = format!("\"{}", first);
    }
    if let Some(last) = wrapped_lines.last_mut() {
        *last = format!("{}\"", last);
    }

    let max_len = wrapped_lines
        .iter()
        .map(|l| UnicodeWidthStr::width(l.as_str()))
        .max()
        .unwrap_or(0)
        .max(UnicodeWidthStr::width(speaker_line.as_str()));

    if add_border {
        let total_width = max_len + 2 * padding;
        let horizontal = "─".repeat(total_width);

        println!("┌{}┐", horizontal);

        for line in &wrapped_lines {
            let padding_right = total_width - UnicodeWidthStr::width(line.as_str()) - padding;
            println!(
                "│{}{}{}│",
                " ".repeat(padding),
                line,
                " ".repeat(padding_right)
            );
        }

        let padding_right =
            total_width - UnicodeWidthStr::width(speaker_line.as_str()) - padding;
        println!(
            "│{}{}{}│",
            " ".repeat(padding),
            speaker_line,
            " ".repeat(padding_right)
        );

        println!("└{}┘", horizontal);
    } else {
        for line in &wrapped_lines {
            println!("{}", line);
        }
        println!("{}", speaker_line);
    }
}

// shit word wrap function
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let tentative = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if UnicodeWidthStr::width(tentative.as_str()) > max_width {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        } else {
            current_line = tentative;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
