mod wincredman;
mod profile;
mod interact;
mod help;
mod winlibs;
mod inject;
// use colored::Colorize;
use crossterm::style::Stylize;
use regex::Regex;
use wincredman::*;
use profile::*;
use interact::*;
use inject::*;
use help::*;
use winlibs::*;
use windows::{Win32::{UI::WindowsAndMessaging::{SetWindowTextA, FindWindowA, GetWindowThreadProcessId}, Foundation::HWND}, core::PCSTR};
use core::time;
use std::{thread,ffi::CString, thread::sleep,process::{Command, ExitStatus}, time::Duration, env};
use clap::{Parser, Subcommand};
use std::fmt;
use std::path::Path;
#[derive(Debug,Clone)]
pub enum Region {
    US,
    EU,
}
impl Default for Region {
    fn default() -> Self {
        Region::US
    }
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
#[derive(Debug,Clone)]
pub enum Mode {
    None,
    Normal,
    Direct,
    DirectTxt,
    Mod,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::None
    }
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::None => write!(f, ""),
            Mode::Normal => write!(f, ""),
            Mode::Direct =>write!(f, "-direct"),
            Mode::DirectTxt => write!(f, "-direct"),
            Mode::Mod => write!(f, "-mod"),
        }
    }
}
/// Diablo II: Resurrected: Awesome Launcher

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///[required for D2RAL Add,Delete,Set-Title]
    #[arg(short = 'n', long = "name-profile", default_value_t = EMPTY_STRING.to_string())]
    name: String,
    ///[required for command: Add](Optional for D2RAL: Edit, Copy)
    #[arg(short, long = "username-profile", default_value_t = EMPTY_STRING.to_string())]
    username: String,
    ///[required for command Add](Optional for D2RAL: Edit, Copy)
    #[arg(short, long = "password-profile", default_value_t = EMPTY_STRING.to_string())]
    password: String,
    //(Optional for D2RAL: Edit, Copy, Start, Volley)
    #[arg(short, long = "region-profile", default_value_t = EMPTY_STRING.to_string())]
    region: String,
    ///[ "none"=Profile , "normal", "direct", "txtdirect" , "{mode}"=-mod {mode} -txt ] (Add, Edit, Copy, Start, Volley) 
    #[arg(short, long = "mode-launch", default_value_t = MODE_NONE.to_string())]
    mode: String,
    /// ["0"=Profile "1"=Sound, "2"=No Sound] (Add, Edit, Copy, Start, Volley)
    #[arg(short, long = "sound-launch", default_value_t = 0)]
    sound: u8,
    ///(Optional for D2RAL: Add, Edit, Copy, Start, Volley) [fullscreen:1 , windowed:2] 
    #[arg(short, long = "window-launch", default_value_t = 0)]
    window: u8,
    // /// Title
    // #[arg(short, long)]
    // title: String,
    #[arg(hide = true, short, long, default_value_t = CONFIRM_NO.to_string())]
    confirm: String,
    /// inject (Optional for D2RAL: Start, Volley) [path to dll]
    #[arg(hide = true,short = 'i', long = "inject", default_value_t = EMPTY_STRING.to_string())]
    dll: String,

    #[command(subcommand)]
    command: D2RAL,
}
#[allow(unused_must_use)]
#[derive(Subcommand,Debug,Clone)]
pub enum D2RAL {
    
    /// D2RAL.exe volley => START ALL THE PROFILES!!!
    Volley,
    /// D2RAL.exe -n {profile_name} start =>  Start a profile
    Start,
    /// D2RAL.exe list => List Stored Profiles
    List,
    /// D2RAL.exe Display {profile_name} => Display Stored Profile Details
    Display { profile: String},
    /// D2RAL.exe -n {profile_name} -u {profile_username} -p {profile_password} add => Add a profile
    Add,
    /// D2RAL.exe -n {profile_name} delete => Delete a profile
    Delete,
    /// D2RAL.exe -n {profile_name} update => update a profile with new options
    Edit,
    /// D2RAL.exe -n {profile_name} copy {new profile name}=> copy a profile with new options
    Copy { new: String},
    /// D2RAL.exe handle => Kill Mulisession mutex handle
    CloseHandle,
    /// D2RAL.exe set-title {new title} => Set a default title window to a new title
    SetTitle { new: String},
    /// D2RAL.exe custom-title {old} {new}
    CustomTitle { old: String, new: String},
    /// D2RAL.exe -i {Dll Path} {window_title} => Inject a compatible Dll into window
    // #[command(hide(true))]
    Inject {dll_path: String, title: String, },
    /// colored examples, 
    Example,
    /// "shell" mode
    Interactive,
    #[command(hide(true))]
    SecretMenuSuperCaliFragiousExpialidocious, 
}

const CONFIRM_NO: &str = "no";
const MODE_NONE: &str = "none";
const MODE_NORMAL: &str = "normal";
const MODE_DIRECT: &str = "direct";
const MODE_DIRECTTXT: &str = "txtdirect";
const EMPTY_STRING: &'static str = "";
const TITLE_NAME: &'static str = "Diablo II: Resurrected";
const TAG: &'static str = "D2R-";

fn main() {
    let cli = cli_prep();
    match &cli.command  {
        D2RAL::Interactive => interactive(),
        _=> command_match(cli.clone()),
    }
    exit()
}
pub fn interactive(){
    loop {
        let answer = into_ask("interactive>");
        let cli2 = Cli::parse_from(answer);
        command_match(cli2.clone());
    }
}

pub fn luggage() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let mut key1:bool = false;
        let mut key2:bool = false;
        let mut key3:bool = false;
        let mut key4:bool = false;
        match args[1].as_str() {
            "luggage" =>{
                key1 = true
            },
            "12345" =>{
                key2 = true
            },
            _=>{
            }
        }
        if args.len() == 3 && (key1 || key2){
            match args[2].as_str() {
                "luggage" =>{
                    key3 = true;
                    args.pop();
                    args.pop();
                },
                "12345" =>{
                    key4 = true;
                    args.pop();
                    args.pop();
                },
                _=>{
                    args.pop();
                    args.pop();
                    key1 = false;
                    key2 = false;
                }
            }
        }
        if (key1 && key4) || (key2 && key3) {
            println!("Secret menu!");
            args.push("secret-menu-super-cali-fragious-expialidocious".to_string());
        }
    }
    check_time();
    args
}

pub fn command_match(cli:Cli){
    match &cli.command {
        D2RAL::List => profiles_list(),
        D2RAL::Add  => profile_add_helper(cli.clone()),
        D2RAL::Edit  => profile_edit_helper(cli.clone()),
        D2RAL::Delete => profile_del_helper(cli.clone()),
        D2RAL::CloseHandle => kill_handle(),
        D2RAL::Start => start(cli.clone()),
        D2RAL::Volley => profile_volley(cli.clone()),
        D2RAL::SetTitle { new } => set_title_helper(&new.clone()),
        D2RAL::CustomTitle { old, new } => custom_title_helper(&old.clone(),&new.clone()),  
        D2RAL::Inject { title, dll_path } => inject_helper(&title.clone(),&dll_path.clone()),
        D2RAL::Copy { new } => profile_copy_helper(cli.clone(),&new.clone()),
        D2RAL::Display { profile } => display_helper(&profile,cli.clone()),
        D2RAL::Example => examples(),
        D2RAL::SecretMenuSuperCaliFragiousExpialidocious => {
            println!("yeet")
        },
        D2RAL::Interactive => {},
    }
}

pub fn exit(){
    std::process::exit(0);
}
pub fn commandparam_to_cli(profile:&String,cli:Cli)->Cli{
    let mut cli =  cli;
    cli.name = profile.to_string();
    name_check(Some(cli.clone())).unwrap()
}
pub fn display_helper(profile:&String,cli:Cli){
    let cli = commandparam_to_cli(profile,cli);
    let profile = profile_select(Some(cli.name.to_string())).unwrap_or_default();
    if profile != Profile::default(){
        list_profile(&profile);
    }
}
pub fn add_check(cli:Option<Cli>)->Option<Cli>{
    name_check(username_check(password_check(region_check(cli))))
}
pub fn name_check(cli:Option<Cli>)->Option<Cli>{
    let cli = cli.unwrap();
    if cli.name == "".to_string() || cli.name == "offline".to_string() {
        println!("--name is empty!");
        exit()
    }
    Some(cli)
}
pub fn username_check(cli:Option<Cli>)->Option<Cli>{
    let cli = cli.unwrap();
    if cli.username == "".to_string() {
        println!("--username is empty! ({})",cli.username);
        exit()
    }
    Some(cli)
}
pub fn password_check(cli:Option<Cli>)->Option<Cli>{
    let cli = cli.unwrap();
    if cli.password == "".to_string() {
        println!("--password is empty! ({})",cli.password);
        exit()
    }
    Some(cli)
}
pub fn region_check(cli:Option<Cli>)->Option<Cli>{
    let cli = cli.unwrap();
    if cli.region == "".to_string() {
        println!("--region is empty!");
        exit()
    }
    Some(cli)
}


pub fn get_mode(mode:&str) -> String {
    match mode {
        MODE_NORMAL => {
            Mode::Normal.to_string()
        },
        MODE_DIRECT => {
            Mode::Direct.to_string()
        },
        MODE_DIRECTTXT => {
            Mode::DirectTxt.to_string()
        },
        MODE_NONE => {
            "".to_string()
        },
        // MODE_MOD => {
        //     Mode::Mod.to_string()
        // },
        _=>{
            // format!("{} {} -txt",Mode::Mod.to_string(),x)
            Mode::Mod.to_string()
        }
    }
}
pub fn get_mod_mode(mode:&str) -> String {
    match mode {
        MODE_NORMAL => {
            "".to_string()
        },
        MODE_DIRECT => {
            "".to_string()
        },
        MODE_DIRECTTXT => {
            "-txt".to_string()
        },
        MODE_NONE => {
            "".to_string()
        },
        x @ _=>{
            format!("{},",x)
        }
    }
}
pub fn get_ext_mode(mode:&str) -> String {
    match mode {
        MODE_NORMAL => {
            "".to_string()
        },
        MODE_DIRECT => {
            "".to_string()
        },
        MODE_DIRECTTXT => {
            "".to_string()
        },
        MODE_NONE => {
            "".to_string()
        },
        _=>{
            "-txt,".to_string()
        }
    }
}

pub fn cli_prep()->Cli{
    let args = luggage();
    let cli = Cli::parse_from(args);
    if cli.name.len() > 0 && cli.name.len() < 2 {
        println!("{}","Please user a Profile name of atleast 2 characters or more");
        exit()
    }
    cli
}