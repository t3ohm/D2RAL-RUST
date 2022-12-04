use crate::*;
pub fn inject_helper(title:&str, dll_path:&str){
    if !verify_inject(title, dll_path) {
        println!("womp");
        return;
    }
    println!("injecting into Window({}) \nwith({})",title,dll_path);
    inject(title,dll_path.into()).expect("Unable to inject!");
    // inject_by_pid(pid_from_title(title), dll_path.into()).expect("Unable to inject!")
}
pub fn verify_inject(title:&str, dll_path:&str)->bool{
    let pid = pid_from_title(title);
    if check_pid(pid) || check_path(dll_path) {
        true
    } else {
        false
    }
}
pub fn check_pid(pid:u32)->bool{
    if pid != 0 {
        true
    } else {
        println!("pid 0");
        false
    }
}
pub fn check_path(path:&str)->bool{
    if Path::new(path).exists(){
        true
    } else {
        println!("path({})",path);
        false
    }
}

pub fn kill_handle(){
    let mut handle_pid = None;
    let mut handle_event = None;
    let output = {
        std::process::Command::new("handle64")
            .args(&["-nobanner","-a","-p","D2R.exe","Instances"])
            .output()
            .expect("failed to execute process")
    };
    let stdout_text = String::from_utf8_lossy(&output.stdout).to_string();
    let re = Regex::new(r".*pid:[ ](?P<p>\d*)[ ]*type: Event[ ]*(?P<e>[A-Za-z0-9]*):.*").unwrap();
    re.captures_iter(&stdout_text).for_each(|cap| {
        handle_pid = Some(cap[1].to_owned());
        handle_event = Some(cap[2].to_owned());
    });
    if handle_pid != None {
        thread::spawn(move || -> Result<ExitStatus, std::io::Error> {
            runas::Command::new("handle64").args(&["-nobanner","-p",&handle_pid.unwrap(),"-c",&handle_event.unwrap(),"-y"]).status()
        });
    }
    sleep(time::Duration::new(0,500000000));
}
pub fn get_pid(hwnd:HWND)->u32{
    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        pid
    }
}
pub fn pid_from_title(title:&str) -> u32 {
    get_pid(get_hwnd(title))
}
pub fn get_hwnd(title_str:&str) -> windows::Win32::Foundation::HWND {
    let title = CString::new(title_str).unwrap();
    unsafe {
        FindWindowA(PCSTR::null(), PCSTR::from_raw(title.as_bytes().as_ptr()))
    }
}
pub fn set_title_helper(new:&str){
    if new.len() == 0 {
        println!("{}","please provide a title with -t <title>");
        exit()
    }
    setwindow_orig(new.to_string());
}
pub fn custom_title_helper(old:&str,new:&str){
    println!("changing window title of {} to {}",old,new);
    setwindow(&old, new.to_string());
}
pub fn setwindow(title_str:&str,new_title:String){
    unsafe {
        let hwnd = {
            get_hwnd(title_str)
        };
        println!("{:?}",hwnd);
        let new_title_cs = CString::new(new_title).unwrap();
        let lpstring = PCSTR::from_raw(new_title_cs.as_bytes().as_ptr());
        SetWindowTextA(hwnd,lpstring);
        sleep(Duration::new(0,500000000));
    };
}
pub fn setwindow_orig(new_title:String){
    unsafe {
        let hwnd = {
            let orig_title = CString::new(TITLE_NAME).unwrap();
            FindWindowA(PCSTR::null(), PCSTR::from_raw(orig_title.as_bytes().as_ptr()))
        };
        let title = CString::new(new_title).unwrap();
        let lpstring = PCSTR::from_raw(title.as_bytes().as_ptr());
        SetWindowTextA(hwnd,lpstring);
        sleep(Duration::new(0,500000000));
    };
}
