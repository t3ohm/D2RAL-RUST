use crate::wincredman::{Credential,read_credential,read_cred_generic, delete_credential};
use std::fmt;
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
    pub fn delete(profile:String){
        delete_credential(&profile)
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