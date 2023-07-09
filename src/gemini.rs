use std::process;

pub mod response_codes {
    pub const INPUT: usize = 10;
    pub const REDIRECT: usize = 30;
    pub const CGI_ERROR: usize = 42;
    pub const BAD_REQUEST: usize = 59;
}

pub fn input(prompt: &str) -> ! {
    print!("{} {}\r\n", response_codes::INPUT, prompt);
    process::exit(0);
}

pub fn bad_input(message: &str) -> ! {
    print!("{} {}\r\n", response_codes::BAD_REQUEST, message);
    process::exit(0);
}

pub fn redirect(destination: &str) -> ! {
    print!("{} {}\r\n", response_codes::REDIRECT, destination);
    process::exit(0);
}

pub fn server_error(message: &str) -> ! {
    print!("{} {}\r\n", response_codes::CGI_ERROR, message);
    process::exit(0);
}
