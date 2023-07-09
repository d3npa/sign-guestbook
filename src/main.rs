use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use urlencoding as ue;

mod gemini;

const GUESTBOOK: &str = "../guestbook.gmi";

const PROMPT: &str = "enter message (please sign your name!)";
const WRITE_ERROR: &str = "error writing to guestbook";

fn write_guestbook(message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(GUESTBOOK)?;

    file.write_all(format!("{}\n", message).as_bytes())?;

    gemini::redirect(GUESTBOOK);

    Ok(())
}

fn main() {
    if let Ok(message) = env::var("QUERY_STRING") {
        let message = match ue::decode(&message) {
            Ok(message) => message,
            Err(_) => {
                gemini::server_error(WRITE_ERROR);
                return;
            }
        };

        if message.is_empty() {
            gemini::input(PROMPT);
        }

        for line in message.split('\n') {
            if line.trim().starts_with("```") {
                continue;
            }

            // lazy patch
            // i really want to remove all ctrl/non-printable chars
            if line.contains('\r') {
                continue;
            }

            if write_guestbook(line).is_err() {
                gemini::server_error(WRITE_ERROR);
            }
        }
    } else {
        gemini::input(PROMPT);
    }
}
