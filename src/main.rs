mod wincredman;
mod winlib;
mod dafuncs;
mod profile;
mod profiles;
mod interact;
use colored::Colorize;
use wildmatch::WildMatch;
use wincredman::*;
use dafuncs::*;
use profile::*;
use profiles::*;
use interact::*;
use winlib::set_window_title;
use core::time;
use std::{env, ffi::CString, thread::sleep};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let mut app = D2RAL::default();
        D2RAL::main(&mut app,args.clone());
    } else {
        D2RAL::help();
    }
}
pub struct D2RAL {
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
        }
    }
}
impl D2RAL {
    pub fn main(d2ral:&mut D2RAL,arguments:Vec<String>){
        d2ral.stored_profiles = Some(Profiles::load_to_vec());
        Self::list_stored_profiles(d2ral);
        Self::prompt_match(d2ral,arguments[1].clone());
    }
    pub fn interactive(d2ral:&mut D2RAL){
        let mut prompt_reponse:String;
        println!("{}","interactive mode:".green());
        loop {
            prompt_reponse = prompt();
            Self::prompt_match(d2ral,prompt_reponse);
        }
    }
    pub fn prompt_match(d2ral:&mut D2RAL,prompt_reponse:String){
        match prompt_reponse.as_str() {
            "-i" | "m" | "menu" => Self::interactive(d2ral),
            "list" | "l" => Self::list_stored_profiles(d2ral),
            "help" | "h" => Self::help(),
            "add" | "a" => Self::addprofile(),
            // "edit" | "e" => Self::addprofile(),
            "del" | "delete" | "d" | "-d" => Self::delprofile(),
            "quit" | "q" => Self::exit(),
            "start" | "s" => {
                Self::prep(d2ral);
                Self::start_profile(d2ral)
            },
            "direct" => {
                Self::prep(d2ral);
                Self::start_profile(d2ral)
            },
            "mod" => {
                Self::prep(d2ral);
                Self::start_profile(d2ral)
            },
            "normal" => {
                Self::prep(d2ral);
                Self::start_profile(d2ral)
            },
            "load" => {
                println!("reloading stored profiles");
                d2ral.stored_profiles = Some(Profiles::load_to_vec());
                Self::list_stored_profiles(d2ral);
            },
            "volley" => {
                for profile in d2ral.stored_profiles.clone().unwrap() {
                    if profile != Profile::default() {
                        d2ral.current_profile = Some(profile);
                        Self::start_profile(d2ral);
                        sleep(time::Duration::from_secs(2));
                    }
                }         

            },
            "test" => {
                               

            },
            // "show" => D2RAL::show(),
            _ => {
                //println!("");
                Self::list_stored_profiles(d2ral);
            }
        }
    }
    
    pub fn load(profile:String)->Option<Credential>{
        println!("{:#?}",read_cred_generic(&format!("D2R-{}",profile)));
        let credential = read_cred_generic(&"D2R-test");
        Some(credential)

    }
    pub fn list_stored_profiles(d2ral:&mut D2RAL){
        d2ral.stored_profiles.as_ref().unwrap().into_iter().for_each(|profile| {
            if profile.name != Profile::default().name {
                println!("{}{}","Profile:".red(),profile.name.yellow());
            }

        });
    }
 

    pub fn start_profile(d2ral:&mut D2RAL){
        print!("spawning profile {}",d2ral.current_profile.as_ref().unwrap().name);
        spawn_d2r_creds(d2ral.current_profile.as_ref().unwrap().credentials.clone());
        sleep(time::Duration::from_secs(3));
        handle_close_d2r(d2ral);
        let new_title =CString::new(d2ral.current_profile.as_ref().unwrap().name.clone()).unwrap();
        set_window_title(new_title);
    }

    pub fn prep(d2ral:&mut D2RAL){
        let old_profile = d2ral.current_profile.clone();
        d2ral.current_profile = None;
        Self::profile_select(d2ral);
        if d2ral.current_profile != None {
            // println!("{:#?}",d2ral.current_profile);
        } else {
            d2ral.current_profile = old_profile;
        }
    }
    pub fn help() {
        let mut help_string:String;
        help_string = format!("-help\t|> all this stuff").red().to_string();
        help_string = format!("{}\nCredential Commands",help_string);
        help_string = format!("{}\n-add\t|> prompt profile: name, username, password",help_string);
        help_string = format!("{}\n-del\t|> prompt profile: name",help_string);
        help_string = format!("{}\n-list\t|> Display profiles in credential manager",help_string);
        help_string = format!("{}\n-quit\t|> quit?",help_string);
        println!("{}",help_string);
    }
    
    pub fn list(){
        println!("{}",Profiles::list_pretty());
    }
    pub fn addprofile(){
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
    }
    pub fn delprofile(){
        let mut unlock =  false;
        loop {
            if unlock {
                break
            }
            let profile = user_input("Type the Profile name you want to delete.>");
            println!("Are you sure you want to delete profile: {}",&profile.red());
            println!("({})es/({})o/({})etry","Y".green(),"N".red(),"R".yellow());
            let engage = user_input("y/n/r_>").to_lowercase();
            if engage == "y" {
                println!("{}{}","Deleting profile: ".red(),profile);
                Profile::delete(profile);
                unlock =  true;
                break
            }
            if engage == "n" {
                println!("{}profile delete","canceling ".red());
                unlock =  true;
                break
            }
            if engage == "r" {
                //do nothing and let it loop
                println!("starting over");
            }
        }
    }
    pub fn profile_select(d2ral:&mut D2RAL){
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
                return
            }
            profile_number+=1;
        }

    }
    pub fn exit(){
        std::process::exit(0);
    }
}

