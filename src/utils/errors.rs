use std::process::exit;

pub fn error_and_exit(content: &str) {
    println!("Convex internal error: {}", content);
    exit(255);
}