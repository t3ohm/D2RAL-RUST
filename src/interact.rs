use std::io::Write;
use colored::Colorize;

pub fn user_input(que:&str) -> String {
    print!("{}", format!("{}",&que).yellow());
    let mut line = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}
// pub fn prompt(que:&str) -> String {
//    user_input(que)
// }

pub fn ask(question:&str) -> String {
    let que = format!("{}>",question);
    user_input(que.as_str())
}

pub fn into_ask(question:&str) -> Vec<String> {
    let answer = ask(question);

    let mut each_arg:Vec<String> = vec![];
    each_arg.push("d2ral".to_string());
    answer.split(" ").for_each(|arg| {
        each_arg.push(arg.to_string());
    });
    each_arg
}

//  pub fn confirm_ynr()-> String {
//     println!("({})es/({})o/({})etry","Y".green(),"N".red(),"R".yellow());
//         ask("y/n/r_>").to_lowercase()
// }