use chrono::DateTime;
use colored::Colorize;

use crate::exit;
pub const TIME:&'static str = "Sun, 11 Dec 2022 00:00:00 +0200";
pub fn check_time(){
    let ct = chrono::offset::Local::now();
    let et = DateTime::parse_from_rfc2822(TIME).unwrap();
    if ct > et {
        println!("{}","you need to update to continue using".red());
        exit()
    }
}
pub fn help_helper(helper:String)->String{
    format!("{}{}",">D2RAL.exe ".red(),helper)
}
pub enum Colors {
    F,
    P,
    C
}
pub fn ci(str_:&str, color:Colors)-> colored::ColoredString {
    match color {
        Colors::F => str_.blue(),
        Colors::P => str_.yellow(),
        Colors::C => str_.purple(),
    }
}

pub fn examples(){
    // let info = "
    let _default_help = "
    Diablo II: Resurrected: Awesome Launcher

    Usage: d2ral.exe [OPTIONS] <COMMAND>
    
    Commands:
      volley        D2RAL.exe volley => START ALL THE PROFILES!!!
      start         D2RAL.exe -n {profile_name} start =>  Start a profile
      list          D2RAL.exe list => List Stored Profiles
      display       D2RAL.exe Display {profile_name} => Display Stored Profile Details
      add           D2RAL.exe -n {profile_name} -u {profile_username} -p {profile_password} add => Add a profile
      delete        D2RAL.exe -n {profile_name} delete => Delete a profile
      edit        D2RAL.exe -n {profile_name} update => update a profile with new options
      copy          D2RAL.exe -n {profile_name} copy {new profile name}=> copy a profile with new options
      close-handle  D2RAL.exe handle => Kill Mulisession mutex handle
      set-title     D2RAL.exe set-title {new title} => Set a default title window to a new title
      custom-title  D2RAL.exe custom-title {old} {new}
      inject        D2RAL.exe -i {Dll Path} {window_title}=> Inject a compatible Dll into window
      help-more     more help
      help          Print this message or the help of the given subcommand(s)
    
    Options:
      -n, --name <NAME>          Profile name     [required for commands Add,Delete,Set-Title] [default: ]
      -u, --username <USERNAME>  Profile username [required for command Add] [default: ]
      -p, --password <PASSWORD>  Profile password [required for command Add] [default: ]
      -r, --region <REGION>      Region [default: ]
      -m, --mode <MODE>          Profile Mode [default: none]
      -s, --sound <SOUND>        sound [default: 0]
      -w, --window <WINDOW>      [fullscreen:1 , windowed:2] [default: 0]
      -c, --confirm <CONFIRM>    Confirm [default: no]
      -i, --inject <INJECT>      inject [default: ]
      -h, --help                 Print help information
      -V, --version              Print version information
";
    let name_flag = ci("-n", Colors::F);
    let user_flag = ci("-u", Colors::F);
    let pass_flag = ci("-p", Colors::F);
    let region_flag = ci("-r", Colors::F);
    let sound_flag = ci("-s", Colors::F);
    let window_flag = ci("-w", Colors::F);
    let mode_flag = ci("-m", Colors::F);
    let inject_flag = ci("-i", Colors::F);
    let example_name = ci("profile1", Colors::P);
    let example_name2 = ci("profile2", Colors::P);
    let example_name3 = ci("profile3", Colors::P);
    let example_user = ci("user@gmail.com", Colors::P);
    let example_user2 = ci("user2@gmail.com", Colors::P);
    let example_pass = ci("12345", Colors::P);
    let example_pass2 = ci("54321", Colors::P);
    let example_region_us = ci("us", Colors::P);
    let example_region_eu = ci("eu", Colors::P);
    let example_on = ci("1", Colors::P);
    let example_off = ci("2", Colors::P);
    let example_blockhd = ci("blockhd", Colors::P);
    let _example_direct = ci("direct", Colors::P);
    let example_directtxt = ci("txtdirect", Colors::P);
    let add_cmd = ci("add", Colors::C);
    let start_cmd = ci("start", Colors::C);
    let edit_cmd = ci("edit", Colors::C);
    let copy_cmd = ci("copy", Colors::C);
    let _inject_cmd = ci("inject", Colors::C);
    let volley_cmd = ci("volley", Colors::C);
    let delete_cmd = ci("delete", Colors::C);
    let example_inject = ci("path\\to\\dll", Colors::P);
    let example_mode_normal = ci("normal", Colors::P);
    let description = format!("Use the {} to set the {} of each {}",ci("flag options", Colors::F),ci("parameters",Colors::P),ci("command",Colors::C));
    let examples = "Examples>>>";
    let add_desc = "Add a profile with region set to 'us', sound off, window mode, and mod 'blockhd'".green();
    let add_example = 
    help_helper(format!("{name_flag} {example_name} {user_flag} {example_user} {pass_flag} {example_pass} {region_flag} {example_region_us} {sound_flag} {example_off} {window_flag} {example_on} {mode_flag} {example_blockhd} {add_cmd}"));

    let start_desc = "Start it".green();
    let start_example = help_helper(format!("{name_flag} {example_name} {start_cmd}"));

    let start_inject_desc = "Start it with injection".green();
    let start_inject_example = help_helper(format!("{name_flag} {example_name} {inject_flag} {example_inject} {start_cmd}"));

    let edit_desc = "Edit the profile with region to eu, sound on, window fullscreen, and '-direct -txt'".green();
    let edit_example = 
    help_helper(format!("{name_flag} {example_name} {user_flag} {example_user} {pass_flag} {example_pass} {region_flag} {example_region_eu} {sound_flag} {example_on} {window_flag} {example_on} {mode_flag} {example_directtxt} {edit_cmd}"));

    let copy_edit_desc = "Copy and edit it changine launch mode and setting to windowed".green();
    let copy_edit_example = help_helper(format!("{name_flag} {example_name} {window_flag} {example_on} {copy_cmd} {example_name2}"));

    let copy_edit_desc2 = "Copy and edit it more".green();
    let copy_edit_example2 = help_helper(format!("{name_flag} {example_name} {user_flag} {example_user2} {pass_flag} {example_pass2} {mode_flag} {example_mode_normal} {copy_cmd} {example_name3}"));
   
    let volly_inject_desc = "Start all profiles with injection".green();
    let volly_inject_example = help_helper(format!("{inject_flag} {example_inject} {volley_cmd}"));
    
    let delete_desc = "Delete them".green();
    let delete_example1 = help_helper(format!("{name_flag} {example_name} {delete_cmd}"));
    let delete_example2 =     help_helper(format!("{name_flag} {example_name2} {delete_cmd}"));
    let delete_example3 =     help_helper(format!("{name_flag} {example_name3} {delete_cmd}"));

    let help_info = format!(
"   {description}
    {examples}
    {add_desc}
    {add_example}
    {start_desc}
    {start_example}
    {start_inject_desc}
    {start_inject_example}
    {edit_desc}
    {edit_example}
    {copy_edit_desc}
    {copy_edit_example}
    {copy_edit_desc2}
    {copy_edit_example2}
    {volly_inject_desc}
    {volly_inject_example}
    {delete_desc}
    {delete_example1}
    {delete_example2}
    {delete_example3}
    "
    );
    // println!("{default_help}");
    println!("{help_info}");
}
