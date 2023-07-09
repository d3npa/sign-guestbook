use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use urlencoding as ue;

mod gemini;

const GUESTBOOK: &str = "../guestbook.gmi";

const PROMPT: &str = "enter message (please sign your name!)";
const WRITE_ERROR: &str = "error writing to guestbook";
const DECODE_ERROR: &str = "error decoding message";
const UNSAFE_MESSAGE: &str = "your message contains unsafe characters";

/// very lazy sanitation check
fn is_safe(message: &str) -> bool {
    !message.contains("\r")
}

fn write_guestbook(message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(GUESTBOOK)?;

    let time = chrono::Utc::now()
        // hardcoding timezone bc for some reason Local isn't working on my server
        // may be a chroot problem.. not entirely sure
        .with_timezone(&chrono_tz::Japan)
        .format("%-d %B, %Y at %H:%M JST")
        .to_string()
        .to_lowercase();

    let formatted = format!(
        "{}\n> -- {time}\n\n",
        message
            .split("\n")
            .filter(|line| !line.is_empty()) // remove double-newlines bc it looks confusing in some clients
            .map(|line| format!("> {line}\n"))
            .collect::<String>()
            .trim()
    );

    file.write_all(formatted.as_bytes())?;

    Ok(())
}

fn main() {
    let query = match env::var("QUERY_STRING") {
        Ok(q) => q,
        Err(_) => gemini::input(PROMPT),
    };

    let message = match ue::decode(&query) {
        Ok(m) => m,
        Err(_) => gemini::server_error(DECODE_ERROR),
    };

    if message.is_empty() {
        gemini::input(PROMPT);
    }

    if !is_safe(&message) {
        gemini::bad_input(UNSAFE_MESSAGE);
    }

    match write_guestbook(&message) {
        Ok(_) => gemini::redirect(GUESTBOOK),
        Err(_) => gemini::server_error(WRITE_ERROR),
    }
}
