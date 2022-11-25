use crate::D2RAL;
use crate::wincredman::{read_credential, Credential};
use crate::winlib::{self, handle_get};
use std::process::{Command, ExitStatus};
use std::thread::JoinHandle;
use std::{thread, fmt};

#[derive(Debug, Clone, Copy)]
pub enum LaunchType {
    Normal,
    Direct,
    Modified,

}
impl fmt::Display for LaunchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Default for LaunchType {
    fn default() -> Self {
        Self::Normal
    }
}
impl PartialEq for LaunchType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Eq for LaunchType {}

pub fn spawn_handle_exe(ui:&mut D2RAL)-> JoinHandle<Result<ExitStatus, std::io::Error>> {
    //thread::spawn(move || {runas::Command::new(exe).arg(arguments).status()})
    let pid = ui.handle_pid.clone();
    let event = ui.handle_event.clone();
    thread::spawn(move || -> Result<ExitStatus, std::io::Error> {
        runas::Command::new("handle64").args(&["-nobanner","-p",&pid.unwrap(),"-c",&event.unwrap(),"-y"]).status()
    })
}

pub fn handle_close_d2r(d2ral:&mut D2RAL){
        while handle_get("D2R".to_string(),d2ral){
            spawn_handle_exe(d2ral);
            if handle_get("D2R".to_string(),d2ral){
                d2ral.handle_event = Some("".to_string());
                d2ral.handle_pid = Some("".to_string());
                println!("multisession handle closed.");
                break
            }
        } 
}
// pub fn is_d2r_credential(target:String) -> bool {
//     is_credential(format!("D2R-{}",target))
// }
// pub fn spawn_d2r(){
//     thread::spawn(move || {
//         let exe_path = "C:\\Program Files (x86)\\Diablo II Resurrected\\D2R.exe";
//         Command::new(exe_path)
//         .args(&["-w","-direct-txt"])
//         .spawn()
//         });
// }
pub fn spawn_d2r_creds(credential:Credential){
    println!("{:#?}",credential.username);
    thread::spawn(move || {
        let exe_path = "C:\\Program Files (x86)\\Diablo II Resurrected\\D2R.exe";
        Command::new(exe_path)
        .args(&["-w","-direct-txt",
            "-Username", &format!("{}",credential.username),
            "-password", &format!("{}",credential.secret),
            "-address", "us.actual.battle.net"])
        .spawn()
        });
}
// pub fn spawn_creds(target:&String){
//     let check = format!("D2R-{}",target);
//     if !is_credential(check.clone()) {
//         return;
//     }
//     spawn_d2r_creds(get_d2r_credentail(target.to_string()));
// }
// pub fn get_d2r_credentail(target:String) -> Credential {
//     let profile = format!("D2R-{}",target);
//     let credential:Credential = read_credential(&profile);
//     credential
// }
// pub fn spawn_handle_exe(ui:&mut D2RAL)-> JoinHandle<Result<ExitStatus, std::io::Error>> {
//     //thread::spawn(move || {runas::Command::new(exe).arg(arguments).status()})
//     let pid = ui.handle_pid.clone();
//     let event = ui.handle_event.clone();
//     thread::spawn(move || -> Result<ExitStatus, std::io::Error> {
//         runas::Command::new("handle64").args(&["-nobanner","-p",&pid,"-c",&event,"-y"]).status()
//     })
// }

// pub fn find_d2r(title: String) -> u32 {
    //     let title = CString::new(title).unwrap();
    //     let pid = unsafe {
        //         let handle = FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()));
        //         let mut pid: u32 = 0;
        //         GetWindowThreadProcessId(handle, Some(&mut pid))
        //     };
        //     pid
        // }
        
        
        
        
        
        
        
