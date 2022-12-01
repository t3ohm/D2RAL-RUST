use crate::wincredman::{Credential,read_credential,read_cred_generic, delete_credential};
use std::fmt;
use std::str;
use serde::__private::from_utf8_lossy;
use wildmatch::WildMatch;
use regex::Regex;
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub credentials: Credential,
    pub state: ProfileState,
    pub role: ProfileType,
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
            state: Default::default(), 
            role: Default::default() }
    }
}
impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Profile {}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProfileType {None,Main,Support}
impl Default for ProfileType {
    fn default() -> Self {
        Self::None
    }
}
impl Profile {
    pub fn new(name: String, credentials: Credential, state: ProfileState, role: ProfileType) -> Self { Self { name, credentials, state, role } }

    pub fn detag(profile:&String)-> String{
            Regex::new(r"D2R-(.*)").unwrap().replace(profile, "$1").to_string()
    }
    pub fn load(profile:String) -> Profile {
        Profile {
            name:Self::detag(&profile),
            credentials:read_cred_generic(&profile),
            state:ProfileState::default(),
            role:ProfileType::None
        }
    }
    pub fn delete(target:String){
        delete_credential(&target)
    }
    pub fn state(){

    }
}
impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum ProfileState {None,Idle,Active,Stopped,Disabled}
impl Default for ProfileState {
    fn default() -> Self {Self::Stopped}
}

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
        let profile_list = Self::list();
        let pretty = from_utf8_lossy(&profile_list).to_string();
        if format!("{:?}",profile_list) == "failed to execute process"{
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


