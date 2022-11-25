use std::io::Write;
use colored::Colorize;

pub fn user_input(face:&str) -> String {
    let mut line = String::new();
    print!("{}", format!("{}",&face).yellow());
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}
pub fn prompt() -> String {
   user_input("D2RAL>")
}
