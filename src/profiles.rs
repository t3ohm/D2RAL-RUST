use crate::{winlib::is_credential, profile::Profile};
use colored::Colorize;
use serde::__private::from_utf8_lossy;
use wildmatch::WildMatch;
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
                        if is_credential(linesplit2.to_string()){
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
    // pub fn display_cred_entries()->String {
    //     let creds_output = Self::list_pretty();
    //     let mut profile_display:String = "".to_string();
    //     if format!("{:?}",creds_output) == "failed to execute process"{
    //         return profile_display
    //     }
    //     creds_output.split("\r\n").for_each(|line: &str| {
    //         for linesplit in line.split("\n"){
    //             for linesplit2 in linesplit.split(": "){
    //                 if WildMatch::new("D2R-*").matches(linesplit2){
    //                     if is_credential(linesplit2.to_string()){
    //                         let profile:Profile = Profile::load(linesplit2.to_string());
    //                         profile_display = format!("{}\n{}",profile_display,profile.name);
    //                     }
    //                 }
    //             }
    //         }
    //     });
    //     format!("Stored Profiles:{}",profile_display)
    // }
}