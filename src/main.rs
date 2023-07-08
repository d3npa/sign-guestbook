use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use urlencoding as ue;

const GUESTBOOK: &str = "../guestbook.gmi";

mod response_codes {
    pub const INPUT: usize = 10;
    pub const REDIRECT: usize = 30;
    pub const CGI_ERROR: usize = 42;
    // pub const BAD_REQUEST: usize = 59;
}

fn prompt_message() {
    print!("{} enter message:\r\n", response_codes::INPUT);
}

// fn bad_input() {
//     print!("{} blacklisted characters\r\n", response_codes::BAD_REQUEST);
// }

fn redirect() {
    print!("{} {}\r\n", response_codes::REDIRECT, GUESTBOOK);
}

fn write_guestbook(message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(GUESTBOOK)?;

    file.write_all(format!("{}\n", message).as_bytes())?;

    redirect();

    Ok(())
}

fn server_error() {
    print!(
        "{} error writing to guestbook\r\n",
        response_codes::CGI_ERROR
    );
}

fn main() {
    if let Ok(message) = env::var("QUERY_STRING") {
        let message = match ue::decode(&message) {
            Ok(message) => message,
            Err(_) => {
                server_error();
                return;
            }
        };

        if message.is_empty() {
            prompt_message();
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
                server_error();
            }
        }
    } else {
        prompt_message();
    }
}
