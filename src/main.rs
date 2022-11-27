mod wincredman;
mod winlib;
mod dafuncs;
mod profile;
mod profiles;
mod interact;
use colored::Colorize;
use regex::Regex;
use wildmatch::WildMatch;
use wincredman::*;
use dafuncs::*;
use profile::*;
use profiles::*;
use interact::*;
use winlib::set_window_title;
use core::time;
use std::{thread,env, ffi::CString, thread::sleep,process::{Command, ExitStatus}};


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = D2RAL::default();
    if args.len() >= 2 {
        D2RAL::main(&mut app,args.clone());
    } else {
        D2RAL::d2ral_cmds(&mut app,Funks::Remove)
    }
}
pub struct D2RAL {
    debug:bool,
    stored_profiles:Option<Vec<Profile>>,
    current_profile:Option<Profile>,
    handle_pid: Option<String>,
    handle_event: Option<String>,
    log: Option<String>,
}
impl Default for D2RAL {
    fn default() -> Self {
        Self { 
            current_profile: Default::default(),
            stored_profiles: Default::default(),
            handle_pid: Some("".to_string()),
            handle_event: Some("".to_string()),
            log: Some(String::with_capacity(9999)),
            debug: false,
        }
    }
}

pub enum Funks {
    Spawn(Profile),
    Start,
    Handle,
    // Function3(LaunchType),
    Select,
    Add,
    Remove,
    Load,
    List,
    Volley,
    Exit,
    Debug,
    Help,
}

impl D2RAL {
    pub fn main(d2ral:&mut D2RAL,arguments:Vec<String>){
        d2ral.stored_profiles = Some(Profiles::load_to_vec());
        // Self::funcs(d2ral,Funks::Exit);
        match arguments[1].clone().to_lowercase().as_str() {
             "-i" | "i" | "interactive" | "m" | "menu" => {
                 loop {Self::prompt_match(d2ral,prompt())}
             },
            _=> Self::prompt_match(d2ral,arguments[1].clone())
        }
    } 
    pub fn prompt_match(d2ral:&mut D2RAL,prompt_reponse:String){
        match prompt_reponse.to_lowercase().as_str() {
            "debug" => Self::d2ral_cmds(d2ral,Funks::Debug),
            "list" | "l" => Self::d2ral_cmds(d2ral,Funks::List),
            "help" | "h" => Self::d2ral_cmds(d2ral,Funks::Help),
            "add" | "a" => Self::d2ral_cmds(d2ral,Funks::Add),
            "del" | "delete" => Self::d2ral_cmds(d2ral,Funks::Remove),
            "quit" | "q" | "exit" => Self::d2ral_cmds(d2ral,Funks::Exit),
            "select" | "sel" => Self::d2ral_cmds(d2ral,Funks::Select),
            "spawn" | "start" | "run" => Self::d2ral_cmds(d2ral,Funks::Start),
            "load" => Self::d2ral_cmds(d2ral,Funks::Load),
            "volley" | "startall" => Self::d2ral_cmds(d2ral,Funks::Volley),
            "test" => {},
            _ => Self::d2ral_cmds(d2ral,Funks::Help)
        }
    }
  
    pub fn d2ral_cmds(d2ral:&mut D2RAL, funks:Funks){
        match funks {
            Funks::Select => {
                // let old_profile = d2ral.current_profile.clone();
                d2ral.current_profile = None;
                let mut profile_number = 0;
                println!("Select profile by number or name");
                let profiles =d2ral.stored_profiles.as_ref().unwrap();
                for profile in profiles {
                    println!("{} -> {}",profile_number.to_string().green(),profile.name.yellow());
                    profile_number+=1;
                }
        
                let profile_selection = user_input("Profile #/name> ");
                profile_number = 0;
                for profile in profiles {
                    
                    if profile_selection == profile_number.to_string() || profile_selection == profile.name {
                        println!("starting profile >  {}", profile.name.red());
                        d2ral.current_profile = Some(profile.clone());
                        break
                    }
                    profile_number+=1;
                }
                if d2ral.current_profile != None {
                    // println!("{:#?}",d2ral.current_profile);
                } else {
                    d2ral.current_profile = Some(Profile::default());
                }
            },
            Funks::Load => {
                println!("reloading stored profiles");
                d2ral.stored_profiles = Some(Profiles::load_to_vec());
                Self::d2ral_cmds(d2ral,Funks::List);
            },
            Funks::Start => {
                Self::d2ral_cmds(d2ral,Funks::Select);
                let profile = d2ral.current_profile.clone().unwrap();
                print!("spawning profile {}",profile.name);
                spawn_d2r_creds(profile.credentials);
                sleep(time::Duration::from_secs(3));
                Self::d2ral_cmds(d2ral,Funks::Handle);
                let new_title =CString::new(profile.name).unwrap();
                set_window_title(new_title);
            },
            Funks::List => {
                d2ral.stored_profiles.as_ref().unwrap().into_iter().for_each(|profile| {
                    if profile.name != Profile::default().name {
                        println!("{}{}","Profile:".red(),profile.name.yellow());
                    }
                });
            },
            Funks::Volley => {
                for profile in d2ral.stored_profiles.clone().unwrap() {
                    if profile != Profile::default() {
                        d2ral.current_profile = Some(profile);
                        Self::d2ral_cmds(d2ral,Funks::Start);
                        sleep(time::Duration::from_secs(2));
                    }
                }
            },
            Funks::Exit => {
                std::process::exit(0);
            },
            Funks::Spawn(value) => {
                
                println!("{:#?}",value.credentials.username);
                 thread::spawn(move || { let exe_path = "C:\\Program Files (x86)\\Diablo II Resurrected\\D2R.exe";
                    Command::new(exe_path)
                    .args(&["-w","-direct-txt",
                        "-Username", &format!("{}",value.credentials.username),
                        "-password", &format!("{}",value.credentials.secret),
                        "-address", "us.actual.battle.net"])
                    .spawn()
                    });},
            Funks::Handle =>{
                let output = {
                    std::process::Command::new("handle64")
                        .args(&["-nobanner","-a","-p","D2R.exe","Instances"])
                        .output()
                        .expect("failed to execute process")
                };
                let stdout_text = String::from_utf8_lossy(&output.stdout).to_string();
                let re = Regex::new(r".*pid:[ ](?P<p>\d*)[ ]*type: Event[ ]*(?P<e>[A-Za-z0-9]*):.*").unwrap();
                re.captures_iter(&stdout_text).for_each(|cap| {
                    d2ral.handle_pid = Some(cap[1].to_owned());
                    d2ral.handle_event = Some(cap[2].to_owned());
                });
                if d2ral.handle_pid.clone().unwrap() != "".to_string(){
                    let pid = d2ral.handle_pid.clone().unwrap();
                    let event = d2ral.handle_event.clone().unwrap();
                    thread::spawn(move || -> Result<ExitStatus, std::io::Error> {
                        runas::Command::new("handle64").args(&["-nobanner","-p",&pid,"-c",&event,"-y"]).status()
                    });
                }

            },
            Funks::Remove => {
                loop {
                    let profile = user_input("Type the Profile name you want to delete.>");
                    println!("Are you sure you want to delete profile: {}",&profile.red());
                    println!("({})es/({})o/({})etry","Y".green(),"N".red(),"R".yellow());
                    let engage = user_input("y/n/r_>").to_lowercase();
                    if engage == "y" {
                        println!("{}{}","Deleting profile: ".red(),profile);
                        Profile::delete(profile);
                        break
                    }
                    if engage == "n" {
                        println!("{}profile delete","canceling ".red());
                        break
                    }
                    if engage == "r" {
                        //do nothing and let it loop
                        println!("starting over");
                    }
                }
            },
            Funks::Add =>{
                let mut unlock:bool =  false;
                let mut engage:String;
                let mut addname:String;
                let mut addusername:String;
                let mut addpassword:String;
                //let mut addpassword_control:String;
                loop {
                    if unlock {
                        break
                    }
                    addname = user_input("Profile name>");
                    loop {
                        addusername = user_input("Profile username>");
                        if WildMatch::new("*@*").matches(&addusername){
                            break
                        }
                        println!("username emails use @...");
                    }
                    addpassword = user_input("Profile password>");
                    // loop {
                    //     addpassword = prompt("Profile password>");
                    //     addpassword_control = prompt("Profile password>");
                    //     if addpassword == addpassword_control {
                    //         break
                    //     }
                    //     println!("{}",format!("passwords do not match").red());
                    // }
                    loop {
                        println!("name:\t\t{}\nusername:\t{}\npassword:\t{}",&addname.red(),&addusername.green(),&addpassword.yellow());
                        println!("({})es/({})o/({})etry","Y".green(),"N".red(),"R".yellow());
                        engage = user_input("y/n/r_>").to_lowercase();
                        if engage == "y" {
                            //add profile and then break
                            println!("{}{}","adding ".green(),"profile");
                            println!("name:\t{}\nusername:\t{}\npassword:\t{}",&addname.red(),&addusername.green(),&addpassword.yellow());
                            let target = format!("D2R-{}",addname);
                            let cred = Credential {
                                target: target.clone(),
                                username: addusername,
                                secret: addpassword,
                                comment: "this is a comment".to_string(),
                                targetalias: "target alias".to_string(),
                                // persist: 2,
                                // attribute_count: 0,
                            };
                            write_credential(cred);
                            unlock =  true;
                            break
                        }
                        if engage == "n" {
                            println!("{}{}","canceling ".red(),"profile");
                            unlock =  true;
                            break
                        }
                        if engage == "r" {
                            //do nothing and let it loop
                            println!("Retrying profile");
                            break
                        }
            
                    }
                }
            },
            Funks::Debug => {
                d2ral.debug = !(d2ral.debug);
                println!("Debug is {}",(if d2ral.debug {"on"} else {"off"}))
            },
            Funks::Help => {
                let mut help_string:String;
                help_string = format!("-help\t|> all this stuff").red().to_string();
                help_string = format!("{}\nCredential Commands",help_string);
                help_string = format!("{}\n-add\t|> prompt profile: name, username, password",help_string);
                help_string = format!("{}\n-del\t|> prompt profile: name",help_string);
                help_string = format!("{}\n-list\t|> Display profiles in credential manager",help_string);
                help_string = format!("{}\n-quit\t|> quit?",help_string);
                println!("{}",help_string);

            },
        }


    }
}

