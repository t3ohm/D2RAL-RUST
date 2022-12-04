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
    /// Profile name     [required for commands Add,Delete,Set-Title]
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    name: String,
    /// Profile username [required for command Add]
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    username: String,
    /// Profile password [required for command Add]
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    password: String,
    /// Region
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    region: String,
    /// Profile Mode
    #[arg(short, long, default_value_t = MODE_NONE.to_string())]
    mode: String,
    // /// mod name
    // #[arg(long)]
    // mod_name: String,
    /// sound
    #[arg(short, long, default_value_t = 0)]
    sound: u8,
    ///[fullscreen:1 , windowed:2]
    #[arg(short, long, default_value_t = 0)]
    window: u8,
    // /// Title
    // #[arg(short, long)]
    // title: String,
    /// Confirm
    #[arg(short, long, default_value_t = CONFIRM_NO.to_string())]
    confirm: String,
    /// inject
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    inject: String,
    
    #[command(subcommand)]
    command: Commands,
}
#[allow(unused_must_use)]
#[derive(Subcommand,Debug,Clone)]
pub enum Commands {
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
    Update,
    /// D2RAL.exe -n {profile_name} copy {new profile name}=> copy a profile with new options
    Copy { new: String},
    /// D2RAL.exe handle => Kill Mulisession mutex handle
    CloseHandle,
    /// D2RAL.exe set-title {new title} => Set a default title window to a new title
    SetTitle { new: String},
    /// D2RAL.exe custom-title {old} {new}
    CustomTitle { old: String, new: String},
    /// D2RAL.exe -i {Dll Path} {window_title} => Inject a compatible Dll into window
    Inject {dll_path: String, title: String, },
    /// more help
    HelpMore,
    // Test, 
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
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "test" =>{
            println!("yeet");
            exit()
        },
        _=>{
            
        }
    }
    let cli = cli_prep();
    match &cli.command {
        Commands::List => profiles_list(),
        Commands::Add  => profile_add_helper(cli),
        Commands::Update  => profile_edit_helper(cli),
        Commands::Delete => profile_del_helper(cli),
        Commands::CloseHandle => kill_handle(),
        Commands::Start => start(cli),
        Commands::Volley => profile_volley(cli),
        Commands::SetTitle { new } => set_title_helper(new),
        Commands::CustomTitle { old, new } => custom_title_helper(old,new),  
        Commands::Inject { title, dll_path } => inject_helper(title,dll_path),
        Commands::Copy { new } => profile_copy_helper(cli.clone(),new),
        Commands::Display { profile } => display_helper(profile,cli.clone()),
        Commands::HelpMore => help_more(),
        // Commands::Test => {
        //     name_check(Some(cli.clone()));
        //     let profile = profile_select(Some(cli.name.to_string())).unwrap_or_default();
        //     println!("{:#?}",profile);
        // },
    }
    std::process::exit(0);
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
    check_time();
    let cli = Cli::parse();
    if cli.name.len() > 0 && cli.name.len() < 2 {
        println!("{}","Please user a Profile name of atleast 2 characters or more");
        exit()
    }
    cli
}