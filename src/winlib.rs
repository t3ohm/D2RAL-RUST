//use crate::D2RAL;
use std::{str, ffi::CString};
use regex::{Regex, Captures};
//use wildmatch::WildMatch;
//use winapi::shared::windef::HWND__;
use windows::{Win32::{UI::WindowsAndMessaging::{SetWindowTextA, FindWindowA}, Foundation::HWND}, core::PCSTR};

use crate::D2RAL;

const EXE_NULL: &'static str = "faux_null.exe";

const EXE_NAME: &'static str = "D2R.exe";

const TITLE_NAME: &'static str = "Diablo II: Resurrected";

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
pub fn handle_get(target:String,d2ral:&mut D2RAL) -> bool {
    let mut handle_pid = String::with_capacity(25);
    let mut handle_event = String::with_capacity(25);
    let output = {
        std::process::Command::new("handle64")
            .args(&["-nobanner","-a","-p",&format!("{}.exe", target),"Instances"])
            .output()
            .expect("failed to execute process")
    };
    let stdout_text = String::from_utf8_lossy(&output.stdout).to_string();
   
    let re = Regex::new(r".*pid:[ ](?P<p>\d*)[ ]*type: Event[ ]*(?P<e>[A-Za-z0-9]*):.*").unwrap();
    re.captures_iter(&stdout_text).for_each(|cap| {
        d2ral.handle_pid = Some(cap[1].to_owned());
        d2ral.handle_event = Some(cap[2].to_owned());
    });
    if d2ral.handle_pid.clone().unwrap() == "".to_string(){
        return false;
    }
    return true
}

pub fn set_window_title(new_title:CString){
    unsafe {
        let hwnd = window_hwnd();
        let lpstring = PCSTR::from_raw(new_title.as_bytes().as_ptr());
        SetWindowTextA(hwnd,lpstring);
    };
}
pub fn empty_hwnd()-> HWND {
    let title = CString::new(EXE_NULL).unwrap();
    unsafe {FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()))}
}
pub fn window_hwnd()-> HWND {
    let title = CString::new(TITLE_NAME).unwrap();
    unsafe {FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()))}
}












////must run app as admin
// pub fn handle_close(ui:&mut D2RAL)-> bool{
//     let status = {
//         std::process::Command::new("handle64")
//             .args(&["-nobanner","-p",&ui.handle_pid,"-c",&ui.handle_event,"-y"])
//             .status()
//             .expect("failed to execute process")
//     };
//     if !status.success() {
//         return false;
//     }
//     return true
// }
// pub fn check_pid_event(ui:&mut D2RAL) -> bool {
//     let re_pid = Regex::new(r"[0-9]*").unwrap();
//     let re_event = Regex::new(r"[A-Za-z0-9]*").unwrap();
//     ui.update(Message::UpdateLog(format!("{}:{}",re_pid,re_event)));
//     if re_pid.is_match(&ui.handle_pid){
//         if re_event.is_match(&ui.handle_event){
//             return false
//         };
//     }
//     true
// }
// pub fn pid_from_title(title:String) -> u32 {
//     let task_list: String = tasklist_by_exe(String::from("D2R.exe"));
//     let mut rdr = csv::Reader::from_reader(task_list.as_bytes());
//     for result in rdr.records() {
//         let record = match result {
//             Ok(it) => it,
//             Err(_) => todo!(),
//         };
//         if title == record[8]{
//             let pid = record[1].parse::<u32>().unwrap();
//             return pid
//         }
//     };
//     0b0
// }
// pub fn set_title(pid: u32){
//     let found_window_final = unsafe { 
//         let found_window:*mut HWND__ = null_mut();
//         let mpid = pid as *mut u32;
//         winuser::GetWindowThreadProcessId(found_window, mpid);
//         found_window
//     };
//     println!("{:?}:{:?}:{:?}",pid,&found_window_final,find_d2r("Diablo II: Resurrected".to_string()));
//     //unsafe { winuser::SetWindowTextA(found_window_final, "test".to_string()) };
// }
// pub fn are_credentials() -> bool {
//     let output = {
//        std::process::Command::new("cmdkey")
//                .arg(format!("/list:D2R-*"))
//                .output()
//                .expect("failed to execute process")
//    };
//    let sub_string = match str::from_utf8(&output.stdout) {
//        Ok(v) => v,
//        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//    };
//    let sub_string = format!("{:#?}",sub_string);
//    if WildMatch::new("*NONE*").matches(&sub_string) || WildMatch::new("*incorrect*").matches(&sub_string){
//        false
//    } else {
//        true
//    }
// }
// pub fn tasklist_by_exe(exe:String) -> String {
//     let output = if cfg!(target_os = "windows") {
//         std::process::Command::new("tasklist")
//                 .args(&["/fi", &format!("IMAGENAME eq {}",exe),"/v","/FO", "CSV"])
//                 .output()
//                 .expect("failed to execute process")
//     } else {
//         std::process::Command::new("sh")
//                 .arg("-c")
//                 .arg("echo Todo!")
//                 .output()
//                 .expect("failed to execute process")
//     };
//     String::from_utf8_lossy(&output.stdout).to_string()
// }