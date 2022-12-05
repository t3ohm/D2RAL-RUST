use crate::*;
use crate::wincredman::{Credential,read_cred_generic,delete_credential};
use std::fmt;
use std::str;
use colored::Colorize;
use serde::__private::from_utf8_lossy;
use wildmatch::WildMatch;
use regex::Regex;
const D2R_LOC: &'static str = "C:\\Program Files (x86)\\Diablo II Resurrected\\D2R.exe";

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub credentials: Credential,
    // pub state: ProfileState,
    // pub role: ProfileType,
}
// impl Into<String> for Profile {
//     fn into(self) -> String {
//         self.name
//     }
// }
impl Default for Profile {
    fn default() -> Self {
        Self { name: "offline".to_string() ,
            credentials:Credential { 
                target:Default::default(),
                username: Default::default(), 
                secret:  Default::default(), 
                comment:  Default::default(),
                targetalias:Default::default(),
                // persist: 2,
                // attribute_count: 0,
            },
            // state: Default::default(), 
            // role: Default::default() 
        }
    }
}
impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Profile {}


// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum ProfileType {None,Main,Support}
// impl Default for ProfileType {
//     fn default() -> Self {
//         Self::None
//     }
// }
impl Profile {
    // pub fn _new(name: String, credentials: Credential, state: ProfileState, role: ProfileType) -> Self { Self { name, credentials, state, role } }

    pub fn detag(profile:&String)-> String{
            Regex::new(r"D2R-(.*)").unwrap().replace(profile, "$1").to_string()
    }
    pub fn load(profile:String) -> Profile {
        Profile {
            name:Self::detag(&profile),
            credentials:read_cred_generic(&profile),
            // state:ProfileState::default(),
            // role:ProfileType::None
        }
    }
    pub fn delete(target:String){
        delete_credential(&target)
    }
    // pub fn _state(){

    // }
}
impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// #[derive(Debug, Copy, Clone)]
// pub enum ProfileState {None,Idle,Active,Stopped,Disabled}
// impl Default for ProfileState {
//     fn default() -> Self {Self::Stopped}
// }

pub struct Profiles {
    pub profiles:Vec<Profile>,
}
impl Profiles {
    pub fn list() -> Vec<u8> {
        let output = std::process::Command::new("cmdkey")
                .arg("/list:D2R-*")
                .output()
                .expect("failed to execute process");
        output.stdout
    }
    pub fn list_pretty()->String {
        let profiles_list = Self::list();
        let pretty = from_utf8_lossy(&profiles_list).to_string();
        if format!("{:?}",profiles_list) == "failed to execute process"{
            return pretty
        }
        pretty
    }
    pub fn load_to_vec()->Vec<Profile> {
        let mut profiles_vec:Vec<Profile> = vec![];
        profiles_vec.push(Profile::default());
        Self::list_pretty().split("\r\n").for_each(|line| {
            for linesplit in line.split("\n"){
                for linesplit2 in linesplit.split(": "){
                    if WildMatch::new("D2R-*").matches(linesplit2){
                        if Self::is_credential(linesplit2.to_string()){
                            // println!("{}{}",&format!("Profile: ").red(),&linesplit2.yellow());
                            //println!("{:#?}",profiles_vec); 
                            profiles_vec.push(Profile::load(linesplit2.to_string()));
                        }
                    }
                }
            }
        });
        profiles_vec
    }
    pub fn is_credential(target:String) -> bool {
        let output = {
           std::process::Command::new("cmdkey")
                   .arg(format!("/list:{}",target))
                   .output()
                   .expect("failed to execute process")
        };
        let s = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let sub_string = format!("{:#?}",s);
        if sub_string.contains("NONE") || sub_string.contains("incorrect"){
            false
        } else {
            true
        }
    }
}

pub fn targetalias_to_args(targetalias:String)->(String,String,String,String,String,String){
    let mut c_cmd:&str = Default::default();
    let mut mode:String = Default::default();
    let mut mode2:String = Default::default();
    let mut mode3:String = Default::default();
    let mut sound: String = Default::default();
    let mut window:String = Default::default();
    let mut d2r_dir:String = Default::default();
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
                    "d2r_dir" => c_cmd="d2r_dir",
                    value @ _=>{
                        if c_cmd != "".to_string().as_str(){
                            match c_cmd {
                                "mode" => mode = value.to_string(),
                                "mode2" => mode2 = value.to_string(),
                                "mode3" => mode3 = value.to_string(),
                                "sound" => sound = "-ns".to_string(),
                                "window" => window = "-w".to_string(),
                                "d2r_dir" => d2r_dir = value.to_string(),
                                _y @ _ => c_cmd = Default::default(),
                                
                            }
                        }
                    }
                }
            }
        }
    });
    (mode,mode2,mode3,sound,window,d2r_dir)
}
pub fn args_to_targetalias(mode:String,sound:String,window:String,d2r_dir:String)->String{
    let mode2 = if mode.is_empty() {format!("")} else {format!("mode:{},",get_mode(&mode))};
    let mod_mode = if mode.is_empty() {format!("")} else {format!("mode2:{},",get_mod_mode(&mode))};
    let ext_mode = if mode.is_empty() {format!("")} else {format!("mode3:{},",get_ext_mode(&mode))};
    let sound = if sound == "1" {format!("")} else if sound == "2" {format!("sound:-ns,")} else {format!("")};
    let window = if window == "1" {format!("")} else if window == "2" {format!("window:-w,")} else {format!("")};
    let d2r_dir = format!("d2r_dir:{},",d2r_dir);
    format!("{}{}{}{}{}{}",mode2,mod_mode,ext_mode,sound,window,d2r_dir)
}
#[allow(unused_assignments)]
pub fn merge_args_to_profile(cli:Cli,profile:Option<Profile>)->Option<Profile>{
    let (mut mode,mod_mode,_ext_mode,mut sound,mut window,mut d2r_dir) = 
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
    if cli.d2r_dir != "".to_string()  {
        d2r_dir = cli.d2r_dir.to_string();
    } else {
        d2r_dir = d2r_dir;
    }
    let mut profile2 = profile.clone().unwrap();
    profile2.credentials.targetalias = args_to_targetalias(mode,sound,window,d2r_dir);
    let profile2 = Some(profile2);
    profile2
}

pub fn profile_copy_edit(cli:Cli,new_name:&str)->Cli{
    let profile = profile_select(Some(cli.clone().name.to_string())).unwrap_or_default();
    let (_mode,mode2,_mode3,sound,window,d2r_dir) = targetalias_to_args(profile.credentials.targetalias.clone());
    let mut cli = cli;
    if profile != Profile::default(){
        cli.name = new_name.to_string();
        cli.username = {
            if cli.username.clone() == EMPTY_STRING.to_string(){
                profile.credentials.username.clone()
            } else {
                cli.username.clone()
            }
        };
        cli.password = {
            if cli.password.clone() == EMPTY_STRING.to_string(){
                profile.credentials.secret.clone()
            } else {
                cli.password.clone()
            }
        };
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
        if cli.d2r_dir != "".to_string() {
            cli.d2r_dir = match d2r_dir.as_str() {
                x @_=> x.to_string(),
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
    let (mode,mode2,mode3,sound,window,d2r_dir) = 
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
    // if cli.confirm.to_lowercase() != "no" {
    //     let check = format!(
    //         "mode:({}) mode2:({}) mode3:({}) sound:({}) window:({}) switch_user:({} {}) switch_pass:({} {}) switch_region:({} {})",
    //         mode,mode2,mode3,sound,window, switch_user, switch_user_value, switch_pass, hide_text(switch_pass_value.clone()), switch_region, switch_region_value
    //     );
    //     ask(&format!("{}\nSpawn?",check));
    // }
    thread::spawn(move || { let exe_path = d2r_dir;
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
    if !cli.dll.is_empty() && check_path(&cli.dll){
        inject_helper(&new_title, &cli.dll)
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
pub fn list_profile(profile:&Profile){
    if profile.name != Profile::default().name {
        let  (mut mode2, mut mod_mode, mut ext_mode, mut sound, mut window,mut d2r_dir)= targetalias_to_args(profile.credentials.targetalias.clone());

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
        if !d2r_dir.is_empty(){
            d2r_dir = format!("{}",d2r_dir)
        }
        let launch_commands = format!("{}{}{}{}{}",mode2.green(),mod_mode.green(),ext_mode.green(),sound.red(),window.yellow());
        let profilename = profile.name.clone().yellow();
        let region = profile.credentials.comment.clone().red();
        println!("Profile({}) region({}) launch_commands({}) d2r-dir({})",profilename.to_string(),region.to_string(),launch_commands,d2r_dir);
    }
}
pub fn profiles_list(){
    Some(Profiles::load_to_vec()).as_ref().unwrap().into_iter().for_each(|profile| {
        list_profile(profile);
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
            cli.window.to_string(),
            cli.d2r_dir.to_string()
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