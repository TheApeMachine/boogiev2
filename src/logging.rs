use colored::*;

pub fn log_info(message: &str) {
    println!("{}", message.blue().bold());
}

pub fn log_warning(message: &str) {
    println!("{}", message.yellow().bold());
}

pub fn log_error(message: &str) {
    eprintln!("{}", message.red().bold());
}
