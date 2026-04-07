//! Output formatting utilities

use colored::Colorize;

/// Print a step message
pub fn step(number: usize, total: usize, message: &str) {
    println!("{} {}", format!("[{}/{}]", number, total).cyan(), message);
}

/// Print a success message
pub fn success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Print a warning message
pub fn warning(message: &str) {
    println!("{} {}", "!".yellow(), message);
}

/// Print an error message
pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Print an info message
pub fn info(message: &str) {
    println!("{} {}", ">".cyan(), message);
}

/// Print a header
pub fn header(title: &str) {
    println!();
    println!("{}", title.bold());
    println!("{}", "─".repeat(title.len()));
}
