use colored::Colorize;
use json::{object, JsonValue};

use crate::{profile::{targetalias_to_args, Profiles, profile_select, Profile}, Cli, get_mode, get_mod_mode, get_ext_mode};
pub struct alias {
  region: String,
  exe: String,
  mode: String,
  window: String,
  sound: String
}


pub fn empty_json_profiles_from_parsed()-> JsonValue {
  json::parse(r#"
  {
    "exe": "path",
    "mode": "normal",
    "window": "default",
    "sound": "default"
  }
  "#).unwrap()

}
pub fn empty_json_profiles_from_obj()-> JsonValue {
   object!{
    // quotes on keys are optional
      exe: "path",
      mode: "normal",
      window: "default",
      sound: "default"
    }
}
pub fn template_alias_obj()-> JsonValue {
  object!{
   // quotes on keys are optional
     exe: "",
     mode: "",
     mode2: "",
     mode3: "",
     window: "",
     sound: ""
   }
}

pub fn cli_to_json(cli:Cli)-> JsonValue{
  
  object!{
    // quotes on keys are optional
      exe: cli.d2r_dir,
      mode: cli.mode.clone(),
      mode2: get_mod_mode(cli.mode.as_str()),
      mode3: get_ext_mode(cli.mode.as_str()),
      window: cli.window,
      sound: cli.sound
  }
}

pub fn merge_cli_alias_json(cli:Cli,targetalias:String) -> JsonValue{
  let (mode,mode2,mode3,sound,window,d2r_dir) = targetalias_to_args(targetalias);
  let mut alias_obj =  object!{};
  if !cli.d2r_dir.clone().is_empty() {
    alias_obj["exe"] =  cli.d2r_dir.into();
  } else {
    alias_obj["exe"] = d2r_dir.into();
  }
  if !cli.region.is_empty() {
    alias_obj["region"] =  cli.region.into();
  } else {
    alias_obj["region"] = "default".into();
  }
  if !cli.mode.clone().is_empty() {
    alias_obj["mode"] =  get_mode(&cli.mode).into();
    alias_obj["mode2"] =  get_mod_mode(&cli.mode).into(); 
    alias_obj["mode3"] =  get_ext_mode(&cli.mode).into();
  } else {
    alias_obj["mode"] = mode.into();
    alias_obj["mode2"] = mode2.into();
    alias_obj["mode3"] = mode3.into();
  }
  if !cli.window != 0 {
    alias_obj["window"] =  cli.window.into();
  } else {
    alias_obj["window"] = window.into();
  }
  if !cli.sound != 0 {
    alias_obj["sound"] =  cli.sound.into();
  } else {
    alias_obj["sound"] = sound.into();
  }
  alias_obj
}



pub fn json_fn(cli:Cli){
  // let parsed = empty_json_profiles_from_parsed();
  // println!("{}",parsed["exe"]);
  // let initiated = empty_json_profiles_from_obj();
  // assert_eq!(parsed, initiated);
  // println!("json matched between\n   parsed: {}\ninitiated: {}", parsed, initiated);
  // for profile in Profiles::load_to_vec(){
  //   let alias_json = merge_cli_alias_json(cli.clone(),profile.credentials.targetalias);
  //   println!("profile_name: {} alias_json{}",profile.name.green(),alias_json);
  // }
  println!("{}",cli_to_json(cli));
}
