use crate::profile::Profile;
use std::str;
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