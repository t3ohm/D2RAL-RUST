use std::io::Write;
use colored::Colorize;

pub fn user_input(que:&str) -> String {
    let mut line = String::new();
    print!("{}", format!("{}",&que).yellow());
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}
pub fn prompt() -> String {
   user_input("D2RAL>")
}

pub fn ask(question:&str) -> String {
    let que = format!("{}>",question);
    user_input(que.as_str())
 }
