mod wincredman;
mod profile;
mod interact;
use colored::Colorize;
use regex::Regex;
use widestring::U16CString;
use wildmatch::WildMatch;
use wincredman::*;
use profile::*;
use interact::*;
use windows::{Win32::UI::WindowsAndMessaging::{SetWindowTextA, FindWindowA, SetWindowTextW}, core::PCSTR};
use core::time;
use std::{thread,env, ffi::CString, thread::sleep,process::{Command, ExitStatus}, collections::hash_map::DefaultHasher, time::Duration};
use clap::{Parser, Subcommand};
use std::fmt;
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
    /// Profile name
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    name: String,
    /// Profile username
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    username: String,
    /// Profile password
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    password: String,
    /// Region
    #[arg(short, long, default_value_t = US_REGION.to_string())]
    region: String,
    /// Profile Mode
    #[arg(short, long, default_value_t = MODE_NONE.to_string())]
    mode: String,
    // /// launch option
    // #[arg(long, default_value_t = EMPTY_STRING.to_string())]
    // launch_option1: String,
    // /// launch option
    // #[arg(long, default_value_t = EMPTY_STRING.to_string())]
    // launch_option2: String,
    // /// launch option
    // #[arg(long, default_value_t = EMPTY_STRING.to_string())]
    // launch_option3: String,
    // /// launch option
    // #[arg(long, default_value_t = EMPTY_STRING.to_string())]
    // launch_option4: String,
    /// mod name
    #[arg(long, default_value_t = EMPTY_STRING.to_string())]
    mod_name: String,
    /// sound
    #[arg(short, long, default_value_t = 0)]
    sound: u8,
    /// window
    #[arg(short, long, default_value_t = 0)]
    window: u8,
    /// Title
    #[arg(short, long, default_value_t = EMPTY_STRING.to_string())]
    title: String,
    /// Confirm
    #[arg(short, long, default_value_t = CONFIRM_NO.to_string())]
    confirm: String,
    
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand,Debug,Clone)]
pub enum Commands {
    /// D2RAL.exe volley => START ALL THE PROFILES!!!
    Volley,
    /// D2RAL.exe -n {profile_name} start =>  Start a profile
    Start,
    /// D2RAL.exe list => List Stored Profiles
    List,
    /// D2RAL.exe -n {profile_name} -u {profile_username} -p {profile_password} add => Add a profile
    Add,
    /// D2RAL.exe -n {profile_name} delete => Delete a profile
    Delete,
    /// D2RAL.exe handle => Kill Mulisession mutex handle
    Handle,
    /// D2RAL.exe -n {profile_name} title => Set a default title window to a new title
    SetTitle,
    /// D2RAL.exe custom-title {old} {new}
    CustomTitle { old: String, new: String},
    // Test,
}
const CONFIRM_NO: &str = "no";
const US_REGION: &str = "us";
const MODE_NONE: &str = "none";
const MODE_NORMAL: &str = "normal";
const MODE_DIRECT: &str = "direct";
const MODE_DIRECTTXT: &str = "txtdirect";
const MODE_MOD: &str = "mod";
const OFFLINE: &'static str = "offline";
const EMPTY_STRING: &'static str = "";
const EXE_NULL: &'static str = "faux_null.exe";
const EXE_NAME: &'static str = "D2R.exe";
const TITLE_NAME: &'static str = "Diablo II: Resurrected";
const TAG: &'static str = "D2R-";

fn main() {
    let args = Cli::parse();
    if args.name.len() > 0 && args.name.len() < 2 {
        println!("{}","Please user a Profile name of atleast 2 characters or more");
        exit()
    }
    match &args.command {
        Commands::Add  => {
            let add_confirmed = add_check(Some(args.clone())).unwrap();
            println!("adding profile > name:{} username:{} region:{} ", add_confirmed.name.red(), add_confirmed.username.green(),add_confirmed.region.cyan());
            let mut cred:Credential = Credential { 
                target: tag(&add_confirmed.name.clone()), 
                username: add_confirmed.username.to_string(), 
                secret: add_confirmed.password.to_string(), 
                comment: add_confirmed.region.to_string(), 
                targetalias: args_to_targetalias(
                    args.mode.to_string(),
                    args.sound.to_string(),
                    args.window.to_string()
                )
            };
            if args.confirm != "no"{
                ask(&format!("{:#?}\nAdd?",cred));
            }
            profile_add(cred);
        },
        Commands::List => {
            profile_list();
        },
        Commands::Delete => {
            name_check(Some(args.clone()));
            let profile = profile_select(Some(args.name.to_string())).unwrap_or_default();
            if profile != Profile::default(){
                println!("Deleting Profile> {}",profile.name.red());
                profile_del(args.name.clone());
            }
        },
        Commands::Handle => {
            kill_handle();
        },
        Commands::Start => {
            let profile = Some(profile_select(Some(args.name.to_string())).unwrap_or_default());
            if profile == None {
                println!("no matching profile found for {}",args.name);
                exit()
            }
            spawn(merge_args_to_profile(args,profile));
        },
        Commands::Volley => {
            profile_volley();
        },
        Commands::SetTitle => {
            if args.title.len() == 0 {
                println!("{}","please provide a title with -t <title>");
                exit()
            }
            setwindow_orig(args.title);
        },
        Commands::CustomTitle { old, new } => {
            println!("changing window title of {} to {}",old,new);
            // let new_old= format!("{}     ",old.clone()).to_string();
            setwindow(&old, new.to_string());
        },
        // Commands::Test => {
            
        // },
    }
    std::process::exit(0);
}

pub fn kill_handle(){
    let mut handle_pid = None;
    let mut handle_event = None;
    let output = {
        std::process::Command::new("handle64")
            .args(&["-nobanner","-a","-p","D2R.exe","Instances"])
            .output()
            .expect("failed to execute process")
    };
    let stdout_text = String::from_utf8_lossy(&output.stdout).to_string();
    let re = Regex::new(r".*pid:[ ](?P<p>\d*)[ ]*type: Event[ ]*(?P<e>[A-Za-z0-9]*):.*").unwrap();
    re.captures_iter(&stdout_text).for_each(|cap| {
        handle_pid = Some(cap[1].to_owned());
        handle_event = Some(cap[2].to_owned());
    });
    if handle_pid != None {
        thread::spawn(move || -> Result<ExitStatus, std::io::Error> {
            runas::Command::new("handle64").args(&["-nobanner","-p",&handle_pid.unwrap(),"-c",&handle_event.unwrap(),"-y"]).status()
        });
    }
    sleep(time::Duration::new(0,500000000));
}
pub fn get_hwnd(title_str:&str) -> windows::Win32::Foundation::HWND {
    let title = CString::new(title_str).unwrap();
    unsafe {
        FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()))
    }
}
pub fn setwindow(title_str:&str,new_title:String){
    unsafe {
        let hwnd = {
            get_hwnd(title_str)
        };
        println!("{:?}",hwnd);
        let new_title_cs = CString::new(new_title).unwrap();
        let lpstring = PCSTR::from_raw(new_title_cs.as_bytes().as_ptr());
        SetWindowTextA(hwnd,lpstring);
        sleep(Duration::new(0,500000000));
    };
}
pub fn setwindow_orig(new_title:String){
    unsafe {
        let hwnd = {
            let orig_title = CString::new(TITLE_NAME).unwrap();
            FindWindowA(PCSTR::null(), PCSTR::from_raw(orig_title.as_bytes().as_ptr()))
        };
        let title = CString::new(new_title).unwrap();
        let lpstring = PCSTR::from_raw(title.as_bytes().as_ptr());
        SetWindowTextA(hwnd,lpstring);
        sleep(Duration::new(0,500000000));
    };
}
pub fn targetalias_to_args(targetalias:String)->(String,String,String,String,String){
    let mut c_cmd:&str = Default::default();
    let mut mode:String = Default::default();
    let mut mode2:String = Default::default();
    let mut mode3:String = Default::default();
    let mut sound: String = Default::default();
    let mut window:String = Default::default();
    targetalias.split(",").for_each(|each| {
        c_cmd = Default::default();
        for cmd in each.split(":"){
            if !cmd.is_empty(){
                match cmd {
                    "mode" => c_cmd="mode",
                    "mode2" => c_cmd="mode2",
                    "mode3" => c_cmd="mode3",
                    "sound" => c_cmd="sound",
                    "window" => c_cmd="window",
                    value @ _=>{
                        if c_cmd != "".to_string().as_str(){
                            match c_cmd {
                                "mode" => mode = value.to_string(),
                                "mode2" => mode2 = value.to_string(),
                                "mode3" => mode3 = value.to_string(),
                                "sound" => sound = "-ns".to_string(),
                                "window" => window = "-w".to_string(),
                                y @ _ => c_cmd = Default::default(),
                                
                            }
                        }
                    }
                }
            }
        }
    });
    (mode,mode2,mode3,sound,window)
}
pub fn args_to_targetalias(mode:String,sound:String,window:String)->String{
    let mode2 = if mode.is_empty() {format!("")} else {format!("mode:{},",get_mode(&mode))};
    let mod_mode = if mode.is_empty() {format!("")} else {format!("mode2:{},",get_mod_mode(&mode))};
    let ext_mode = if mode.is_empty() {format!("")} else {format!("mode3:{},",get_ext_mode(&mode))};
    let sound = if sound == "1" {format!("")} else if sound == "2" {format!("sound:-ns,")} else {format!("")};
    let window = if window == "1" {format!("")} else if window == "2" {format!("window:-w,")} else {format!("")};
    format!("{}{}{}{}{}",mode2,mod_mode,ext_mode,sound,window)
}
pub fn merge_args_to_profile(args:Cli,profile:Option<Profile>)->Option<Profile>{
    let (mut mode,mut mod_mode,mut ext_mode,mut sound,mut window) = 
        targetalias_to_args(profile.clone().unwrap().credentials.targetalias);
    if args.mode.to_lowercase() != "none" {
        mode = args.mode;
    } else {
        println!("{}",mode);
        mode =  match mod_mode.as_str() {
            x @ _=> x.to_string()
        }
    }
    if args.sound != 0 {
        sound = args.sound.to_string();
    } else {
        sound =  match sound.as_str() {
            "-ns" =>"2".to_string(),
            x @ _=> x.to_string() ,
        }
    }
    if args.window != 0  {
        window = args.window.to_string();
    } else {
        window =  match window.as_str() {
            "-w" =>"2".to_string(),
            x @ _=> x.to_string() ,
        }
    }
    let mut profile2 = profile.clone().unwrap();
    profile2.credentials.targetalias = args_to_targetalias(mode,sound,window);
    let profile2 = Some(profile2);
    println!("{:#?}",profile2.as_ref().unwrap());
    profile2
}
pub fn spawn(mut profile:Option<Profile>){
    if profile == None {profile = Some(Profile::default())}
    let profile_spawn_cred = profile.clone().unwrap().credentials;
    let (mode,mode2,mode3,sound,window) = 
        targetalias_to_args(profile_spawn_cred.targetalias.clone());
    if !profile_spawn_cred.username.is_empty() {println!("{}",profile_spawn_cred.username)}
    let mut switch_user:&str = Default::default();
    let mut switch_pass:&str = Default::default();
    let mut switch_region:&str = Default::default();
    let mut switch_user_value:String = Default::default();
    let mut switch_pass_value:String = Default::default();
    let mut switch_region_value:String = Default::default();
    if online_user_check(&profile_spawn_cred){
        switch_user = "-Username";
        switch_pass = "-password";
        switch_region = "-address";
        switch_user_value = profile_spawn_cred.username;
        switch_pass_value = profile_spawn_cred.secret;
        switch_region_value = format!("{}.actual.battle.net",profile_spawn_cred.comment);
    } else {
        
    }
    let check = format!(
        "mode:({}) mode2:({}) mode3:({}) sound:({}) window:({}) switch_user:({} {}) switch_pass:({} {}) switch_region:({} {})",
        mode,mode2,mode3,sound,window, switch_user, switch_user_value, switch_pass, hide_text(switch_pass_value.clone()), switch_region, switch_region_value
    );
    // ask(&format!("{}\nSpawn?",check));
    thread::spawn(move || { let exe_path = "C:\\Program Files (x86)\\Diablo II Resurrected\\D2R.exe";
        Command::new(exe_path)
            .args(&[
                mode.as_str(),mode2.as_str(),mode3.as_str(),
                sound.as_str(),window.as_str(),
                switch_user, &switch_user_value,
                switch_pass, &switch_pass_value,
                switch_region, &switch_region_value
            ]).spawn()
            
    });
    sleep(Duration::new(0,500000000));
    kill_handle();
    if check_name(&profile.clone().unwrap()){
        setwindow_orig(format!("{}{}",TAG,profile.clone().unwrap().name));
    }
}
pub fn profile_list(){
    Some(Profiles::load_to_vec()).as_ref().unwrap().into_iter().for_each(|profile| {
        if profile.name != Profile::default().name {
            println!("Profile({}) launch_commands({}) region({})",profile.name.yellow(),profile.credentials.targetalias.green(),profile.credentials.comment.red());
        }
    });
}
pub fn profile_volley(){
    for profile in Profiles::load_to_vec() {
        if profile != Profile::default() {
            spawn(Some(profile));
            sleep(time::Duration::from_secs(2));
        }
    }
}
pub fn profile_del(target: String){
    Profile::delete(tag(&target));
}
pub fn profile_add(cred:Credential){
    write_credential(cred);
}
pub fn profile_select(p_profile:Option<String>) -> Option<Profile> {
    let mut current_profile = None;
    let mut profile_number = 0;
    let stored_profiles = Profiles::load_to_vec();
    let mut profile_selection:String;
    if p_profile == None {
        println!("Select profile by number or name");
        // let profiles = stored_profiles.as_ref().unwrap();
        for profile in &stored_profiles {
            println!("{} -> {}",profile_number.to_string().green(),profile.name.yellow());
            profile_number+=1;
        }
        profile_selection = ask("Profile #/name> ");

    } else {
        profile_selection = p_profile.unwrap()
    }
    profile_number = 0;
    for profile in &stored_profiles {
        if profile_selection == profile_number.to_string() || profile_selection == profile.name {
            current_profile = Some(profile.clone());
            break
        }
        profile_number+=1;
    }
    current_profile
}
pub fn tag(d2r_:&String)-> String {
    format!("{}{}",TAG,d2r_)
}
pub fn exit(){
    std::process::exit(0);
}

pub fn add_check(cli:Option<Cli>)->Option<Cli>{
    let mut cli = name_check(cli);
    cli = username_check(cli);
    cli = password_check(cli);
    // if cli.unwrap().name == "".to_string() {
    //     return None
    // }
    cli
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
pub fn check_name(pro:&Profile)->bool{
    if pro.name.is_empty(){
        return false
    } else  {
        return true
    }
}
pub fn hide_text(text:String)->String{
    let mut hidden:String = String::default();
    for char in text.chars(){
        hidden.push('*');
    }
    hidden
}
pub fn online_user_check(cred:&Credential)->bool{
    if cred.username.is_empty() && cred.secret.is_empty(){
        return false
    } else  {
        return true
    }
}

pub fn get_mode(mode:&str) -> String {
    let final_mode = match mode {
        MODE_NORMAL => {
            Mode::Normal.to_string()
        },
        MODE_DIRECT => {
            Mode::Direct.to_string()
        },
        MODE_DIRECTTXT => {
            Mode::DirectTxt.to_string()
        },
        // MODE_MOD => {
        //     Mode::Mod.to_string()
        // },
        x @ _=>{
            // format!("{} {} -txt",Mode::Mod.to_string(),x)
            Mode::Mod.to_string()
        }
    };
    // println!("mode:{}", final_mode);
    final_mode
}
pub fn get_mod_mode(mode:&str) -> String {
let final_mode = match mode {
    MODE_NORMAL => {
        "".to_string()
    },
    MODE_DIRECT => {
        "".to_string()
    },
    MODE_DIRECTTXT => {
        "-txt".to_string()
    },
    // MODE_MOD => {
    //     Mode::Mod.to_string()
    // },
    x @ _=>{
        // format!("{} {} -txt",Mode::Mod.to_string(),x)
        format!("{},",x)
    }
};
// println!("mode:{}", final_mode);
final_mode
}
pub fn get_ext_mode(mode:&str) -> String {
    let final_mode = match mode {
        MODE_NORMAL => {
            "".to_string()
        },
        MODE_DIRECT => {
            "".to_string()
        },
        MODE_DIRECTTXT => {
            "".to_string()
        },
        // MODE_MOD => {
        //     Mode::Mod.to_string()
        // },
        x @ _=>{
            // format!("{} {} -txt",Mode::Mod.to_string(),x)
            "-txt,".to_string()
        }
    };
    final_mode
}