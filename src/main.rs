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

fn write_guestbook(message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(GUESTBOOK)?;

    let time = chrono::Local::now().format("%Y/%m/%d %H:%M JST");

    let formatted = format!(
        "{}\n> ({time})\n\n",
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

/// very lazy sanitation check
fn is_safe(message: &str) -> bool {
    if message.contains("\r") {
        false
    } else {
        true
    }
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
