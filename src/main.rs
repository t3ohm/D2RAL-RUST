mod wincredman;
mod profile;
mod interact;
mod inject;
// use colored::Colorize;
use crossterm::style::Stylize;
// use hudhook::inject::inject_by_pid;
use regex::Regex;
use wincredman::*;
use profile::*;
use interact::*;
use inject::*;
use windows::{Win32::{UI::WindowsAndMessaging::{SetWindowTextA, FindWindowA, GetWindowThreadProcessId}, Foundation::HWND}, core::PCSTR};
use core::time;
use std::{thread,ffi::CString, thread::sleep,process::{Command, ExitStatus}, time::Duration};
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
    #[arg(short, long)]
    name: String,
    /// Profile username [required for command Add]
    #[arg(short, long)]
    username: String,
    /// Profile password [required for command Add]
    #[arg(short, long)]
    password: String,
    /// Region
    #[arg(short, long)]
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
    /// help2
    #[arg(long)]
    help2: bool,
    
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
    /// D2RAL.exe -n {profile_name} update => update a profile with new options
    Update,
    /// D2RAL.exe -n {profile_name} copy {new profile name}=> copy a profile with new options
    Copy { new: String},
    /// D2RAL.exe handle => Kill Mulisession mutex handle
    Handle,
    /// D2RAL.exe set-title {new title} => Set a default title window to a new title
    SetTitle { new: String},
    /// D2RAL.exe custom-title {old} {new}
    CustomTitle { old: String, new: String},
    /// D2RAL.exe -i "{Dll Path}" "{window_title}"=> Inject a compatible Dll into window
    Inject {dll_path: String, title: String, },
    /// more help
    HelpMore,
    Test, 
}

// const MODE_MOD: &str = "mod";
// const OFFLINE: &'static str = "offline";
// const EXE_NULL: &'static str = "faux_null.exe";
// const EXE_NAME: &'static str = "D2R.exe";
const CONFIRM_NO: &str = "no";
// const US_REGION: &str = "us";
const MODE_NONE: &str = "none";
const MODE_NORMAL: &str = "normal";
const MODE_DIRECT: &str = "direct";
const MODE_DIRECTTXT: &str = "txtdirect";
const EMPTY_STRING: &'static str = "";
const TITLE_NAME: &'static str = "Diablo II: Resurrected";
const TAG: &'static str = "D2R-";

fn main() {
    let cli = cli_prep();
    match &cli.command {
        Commands::List => profile_list(),
        Commands::Add  => profile_add_helper(cli),
        Commands::Update  => profile_edit_helper(cli),
        Commands::Delete => profile_del_helper(cli),
        Commands::Handle => kill_handle(),
        Commands::Start => start(cli),
        Commands::Volley => profile_volley(cli),
        Commands::SetTitle { new } => set_title_helper(new),
        Commands::CustomTitle { old, new } => custom_title_helper(old,new),  
        Commands::Inject { title, dll_path } => inject_helper(title,dll_path),
        Commands::Test => {
            name_check(Some(cli.clone()));
            let profile = profile_select(Some(cli.name.to_string())).unwrap_or_default();
            println!("{:#?}",profile);
        },
        Commands::Copy { new } => {
            profile_copy_helper(cli.clone(),new);
        },
        Commands::HelpMore => todo!(),
    }
    std::process::exit(0);
}

pub fn inject_helper(title:&str, dll_path:&str){
    if !verify_inject(title, dll_path) {
        println!("womp");
        return;
    }
    println!("injecting into Window({}) \nwith({})",title,dll_path);
    inject(title,dll_path.into()).expect("Unable to inject!");
    // inject_by_pid(pid_from_title(title), dll_path.into()).expect("Unable to inject!")
}
pub fn verify_inject(title:&str, dll_path:&str)->bool{
    let pid = pid_from_title(title);
    if check_pid(pid) || check_path(dll_path) {
        true
    } else {
        false
    }
}
pub fn check_pid(pid:u32)->bool{
    if pid != 0 {
        true
    } else {
        println!("pid 0");
        false
    }
}
pub fn check_path(path:&str)->bool{
    if Path::new(path).exists(){
        true
    } else {
        println!("path({})",path);
        false
    }
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
pub fn get_pid(hwnd:HWND)->u32{
    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        pid
    }
}
pub fn pid_from_title(title:&str) -> u32 {
    get_pid(get_hwnd(title))
}
pub fn get_hwnd(title_str:&str) -> windows::Win32::Foundation::HWND {
    let title = CString::new(title_str).unwrap();
    unsafe {
        FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()))
    }
}
pub fn set_title_helper(new:&str){
    if new.len() == 0 {
        println!("{}","please provide a title with -t <title>");
        exit()
    }
    setwindow_orig(new.to_string());
}
pub fn custom_title_helper(old:&str,new:&str){
    println!("changing window title of {} to {}",old,new);
    setwindow(&old, new.to_string());
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
                                _y @ _ => c_cmd = Default::default(),
                                
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
#[allow(unused_assignments)]
pub fn merge_args_to_profile(cli:Cli,profile:Option<Profile>)->Option<Profile>{
    let (mut mode,mod_mode,_ext_mode,mut sound,mut window) = 
        targetalias_to_args(profile.clone().unwrap().credentials.targetalias);
    if cli.mode.to_lowercase() != "none" {
        mode = cli.mode;
    } else {
        mode =  match mod_mode.as_str() {
            x @ _=> x.to_string()
        }
    }
    if cli.sound != 0 {
        sound = cli.sound.to_string();
    } else {
        sound =  match sound.as_str() {
            "-ns" =>"2".to_string(),
            x @ _=> x.to_string() ,
        }
    }
    if cli.window != 0  {
        window = cli.window.to_string();
    } else {
        window =  match window.as_str() {
            "-w" =>"2".to_string(),
            x @ _=> x.to_string() ,
        }
    }
    let mut profile2 = profile.clone().unwrap();
    profile2.credentials.targetalias = args_to_targetalias(mode,sound,window);
    let profile2 = Some(profile2);
    profile2
}

pub fn profile_copy_edit(cli:Cli,new_name:&str)->Cli{
    let profile = profile_select(Some(cli.clone().name.to_string())).unwrap_or_default();
    let (_mode,mode2,_mode3,sound,window) = targetalias_to_args(profile.credentials.targetalias.clone());
    let mut cli = cli;
    if profile != Profile::default(){
        cli.name = new_name.to_string();
        cli.username = profile.credentials.username.clone();
        cli.password = profile.credentials.secret.clone();
        cli.region = {
            if cli.region.clone() == EMPTY_STRING.to_string(){
                profile.credentials.comment.clone()
            } else {
                cli.region.clone()
            }
        };
        if cli.mode.clone() == MODE_NONE.to_string(){
            cli.mode = mode2;
        }
        if cli.sound == 0 {
            cli.sound = match sound.as_str() {
                "-ns" => 2,
                _=> 1,
            }
        }
        if cli.window == 0 {
            cli.window = match window.as_str() {
                "-w" => 2,
                _=> 1,
            }
        }
    }
    cli
}
pub fn profile_edit_helper(cli:Cli){
    name_check(Some(cli.clone()));
    let cli = profile_copy_edit(cli.clone(),&cli.name.clone());
    println!("Editing profile {}",cli.name);
    profile_add_helper(cli)
}
pub fn profile_copy_helper(cli:Cli,new_name:&str){
    name_check(Some(cli.clone()));
    let copyfrom = cli.name.clone();
    let cli = profile_copy_edit(cli.clone(),&new_name);
    println!("Copying profile {} to {}",copyfrom,cli.name);
    profile_add_helper(cli);
}

pub fn spawn(cli:Cli,mut profile:Option<Profile>){
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
    if cli.confirm.to_lowercase() != "no" {
        let check = format!(
            "mode:({}) mode2:({}) mode3:({}) sound:({}) window:({}) switch_user:({} {}) switch_pass:({} {}) switch_region:({} {})",
            mode,mode2,mode3,sound,window, switch_user, switch_user_value, switch_pass, hide_text(switch_pass_value.clone()), switch_region, switch_region_value
        );
        ask(&format!("{}\nSpawn?",check));
    }
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
    let new_title = format!("{}{}",TAG,profile.clone().unwrap().name);
    if check_name(&profile.clone().unwrap()){
        setwindow_orig(new_title.clone());
    }
    if check_path(&cli.inject){
        inject_helper(&new_title, &cli.inject)
    }

}
pub fn start(cli:Cli){
    let profile = Some(profile_select(Some(cli.name.to_string())).unwrap_or_default());
            if profile == None {
                println!("no matching profile found for {}",cli.name);
                exit()
            }
            spawn(cli.clone(),merge_args_to_profile(cli,profile));
}
pub fn profile_list(){
    Some(Profiles::load_to_vec()).as_ref().unwrap().into_iter().for_each(|profile| {
        if profile.name != Profile::default().name {
            let  (mut mode2, mut mod_mode, mut ext_mode, mut sound, mut window)= targetalias_to_args(profile.credentials.targetalias.clone());

            if !mode2.is_empty(){
                mode2 = format!("{} ",mode2)
            }
            if !mod_mode.is_empty(){
                mod_mode = format!("{} ",mod_mode)
            }
            if !ext_mode.is_empty(){
                ext_mode = format!("{} ",ext_mode)
            }
            if !sound.is_empty(){
                sound = format!("{} ",sound)
            }
            if !window.is_empty(){
                window = format!("{}",window)
            }
            let launch_commands = format!("{}{}{}{}{}",mode2.green(),mod_mode.green(),ext_mode.green(),sound.red(),window.yellow());
            let profilename = profile.name.clone().yellow();
            let region = profile.credentials.comment.clone().red();
            println!("Profile({}) region({}) launch_commands({}) ",profilename.to_string(),region.to_string(),launch_commands);
        }
    });
}
pub fn profile_volley(cli:Cli){
    for profile in Profiles::load_to_vec() {
        if profile != Profile::default() {
            spawn(cli.clone(),merge_args_to_profile(cli.clone(),Some(profile)));
            sleep(time::Duration::from_secs(2));
        }
    }
}
pub fn profile_del(target: String){
    Profile::delete(tag(&target));
}
pub fn profile_del_helper(cli:Cli){
    name_check(Some(cli.clone()));
    let profile = profile_select(Some(cli.name.to_string())).unwrap_or_default();
    if profile != Profile::default(){
        println!("Deleting Profile> {}",profile.name.red());
        profile_del(cli.name.clone());
    }
}
pub fn profile_add_helper(cli:Cli){
    //unwrap wont panic due to exit()
    let cli = add_check(Some(cli.clone())).unwrap();
    println!("adding profile > name:{} username:{} region:{} ", cli.name.clone().red(), cli.username.clone().green(),cli.region.clone().cyan());
    let cred:Credential = Credential { 
        target: tag(&cli.name.clone()), 
        username: cli.username.to_string(), 
        secret: cli.password.to_string(), 
        comment: cli.region.to_string(), 
        targetalias: args_to_targetalias(
            cli.mode.to_string(),
            cli.sound.to_string(),
            cli.window.to_string()
        )
    };
    if cli.confirm != "no"{
        ask(&format!("{:#?}\nAdd?",cred));
    }
    profile_add(cred);
}
pub fn profile_add(cred:Credential){
    write_credential(cred);
}
pub fn profile_select(p_profile:Option<String>) -> Option<Profile> {
    let mut current_profile = None;
    let mut profile_number = 0;
    let stored_profiles = Profiles::load_to_vec();
    let profile_selection:String;
    if p_profile == None {
        println!("Select profile by number or name");
        // let profiles = stored_profiles.as_ref().unwrap();
        for profile in &stored_profiles {
            println!("{} -> {}",profile_number.to_string().green(),profile.name.clone().yellow());
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
pub fn check_name(pro:&Profile)->bool{
    if pro.name.is_empty(){
        false
    } else {
        true
    }
}
pub fn hide_text(text:String)->String{
    let mut hidden:String = String::default();
    for _ in text.chars(){
        hidden.push('*');
    }
    hidden
}
pub fn online_user_check(cred:&Credential)->bool{
    if cred.username.is_empty() && cred.secret.is_empty(){
        false
    } else {
        true
    }
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
    let cli = Cli::parse();
    if cli.name.len() > 0 && cli.name.len() < 2 {
        println!("{}","Please user a Profile name of atleast 2 characters or more");
        exit()
    }
    cli
}