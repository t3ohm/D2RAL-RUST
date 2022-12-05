
use std::{env::{consts::ARCH, self}, path::{Path, PathBuf},io::{Cursor, self}, fs::{File, self}};

use zip::ZipArchive;

use crate::interact::ask;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const HANDLE_URL: &'static str = "https://download.sysinternals.com/files/Handle.zip";
const HANDLE_FILE: &'static str = "handle.zip";

pub fn get_arch() -> &'static str {
    match ARCH {
        "x86_64" | "aarch64" => "x64",
        _=> ""
    }
}

pub(crate) async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
#[tokio::main]
pub(crate) async fn handle_dl(file:String) -> Result<()> {
    fetch_url(HANDLE_URL.to_string(), file).await
}

pub fn handle_prep() -> &'static str{
    // let target_file = std::fs::File::create(target_dir);
    let target_dir = PathBuf::from("./handle/"); // Doesn't need to exist
    if !target_dir.exists(){
        let _ = fs::create_dir_all(target_dir.clone());
    }
    let file_string= format!("{}{HANDLE_FILE}","./handle/");
    let file = PathBuf::from(file_string.clone()); 
    while !file.exists() {
        let _ = handle_dl(file_string.clone());
        if file.exists(){
            break
        }
    }
    let file_path32 = PathBuf::from(format!("./handle/handle.exe"));
    let file_path64 = PathBuf::from(format!("./handle/handle64.exe"));
    while !file_path32.exists() || !file_path64.exists() {
        unzip(file.clone(),target_dir.clone());
        if file_path32.exists() && file_path64.exists(){
            break
        }
    }
    match get_arch() {
        "x86_64" => "./handle/handle64.exe" ,
        _=> "./handle/handle.exe"
    }

    
    // let file = fs::File::open(&file).unwrap();

    // let mut archive = zip::ZipArchive::new(file).unwrap();
    // for i in 0..archive.len() {
    //     let mut file = archive.by_index(i).unwrap();
    //     let outpath = match file.enclosed_name() {
    //         Some(path) => PathBuf::from(format!("{}{}","./handle/",path.display())),
    //         None => continue,
    //     };

    //     // {
    //     //     let comment = file.comment();
    //     //     if !comment.is_empty() {
    //     //         println!("File {} comment: {}", i, comment);
    //     //     }
    //     // }

    //     if (*file.name()).ends_with('/') {
    //         println!("File {} extracted to \"{}\"", i, outpath.display());
    //         fs::create_dir_all(&outpath).unwrap();
    //     } else {
    //         println!(
    //             "File {} extracted to \"{}\" ({} bytes)",
    //             i,
    //             outpath.display(),
    //             file.size()
    //         );
    //         if let Some(p) = outpath.parent() {
    //             if !p.exists() {
    //                 fs::create_dir_all(p).unwrap();
    //             }
    //         }
    //         ask(&format!("{}",outpath.display()));
    //         let mut outfile2 = fs::File::create(&outpath).unwrap();
    //         io::copy(&mut file, &mut outfile2).unwrap();
    //     }
    // }
    // if PathBuf::from("./handle/handle.exe").exists(){
    //     true
    // } else {
    //     false
    // }
}

pub fn unzip(file:PathBuf,out_dir:PathBuf)->bool{
    let file = fs::File::open(&file).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => PathBuf::from(format!("{}{}",out_dir.display(),path.display())),
            None => continue,
        };

        // {
        //     let comment = file.comment();
        //     if !comment.is_empty() {
        //         println!("File {} comment: {}", i, comment);
        //     }
        // }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            // ask(&format!("{}",outpath.display()));
            let mut outfile2 = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile2).unwrap();
        }
    }
    if PathBuf::from("./handle/handle.exe").exists(){
        true
    } else {
        false
    }

}
// HandlePrep(){
//     static path
//     if path ~="handle"
//         return path
//     if !(((this.stdout((handleexe:="handle" utils.GetArch() ".exe") " -h") != (CNF:="Command not found"))?(handlepath:=handleexe):(this.stdout(tp:=A_ScriptDir "\" (handledir:="Handle") "\" handleexe " -h") != CNF )?(handlepath:=tp):(handlepath:=false)))
//     {
//         UrlDownloadToFile, match ARCH {
    //     "x86_64" | "aarch64" => "x64",
    //     _=> ""
    // } , % file:="Handle.zip"
//         utils.Extract2Folder(file,handledir)
//         if FileExist(handledir "\Eula.txt" )
//         {
//             FileMove, % handledir "\Eula.txt", % handledir "\handle-Eula.txt", 1
//             this.stdout(tp " -accepteula") 
//             return path:=tp
//         }
//     } else {
//         return path:=handlepath
//     }
// }

// GetArch(){
//     for sysinfo in ComObjGet("winmgmts:\\.\root\cimv2").ExecQuery("SELECT OSArchitecture FROM Win32_OperatingSystem")
//         return sysinfo.OSArchitecture="64-bit"?"64":""
// }

// Extract2Folder(Zip, Dest="", Filename=""){
//     /*
//         Zip (required)
//             If no path is specified then Zip is assumed to be in the Script Folder
//         Dest (optional)
//             Name of folder to extract to
//             If not specified, a folder based on the Zip name is created in the Script Folder
//             If a full path is not specified, then the specified folder is created in the Script Folder
//         Filename (optional)
//             Name of file to extract
//             If not specified, the entire contents of Zip are extracted
//             Only works for files in the root folder of Zip
//             Wildcards not allowed
//     */
//     SplitPath, Zip,, SourceFolder
//     if ! SourceFolder
//         Zip := A_ScriptDir . "\" . Zip
    
//     if ! Dest {
//         SplitPath, Zip,, DestFolder,, Dest
//         Dest := DestFolder . "\" . Dest . "\"
//     }
//     if SubStr(Dest, 0, 1) <> "\"
//         Dest .= "\"
//     SplitPath, Dest,,,,,DestDrive
//     if ! DestDrive
//         Dest := A_ScriptDir . "\" . Dest
    
//     fso := ComObjCreate("Scripting.FileSystemObject")
//     If Not fso.FolderExists(Dest)  ;http://www.autohotkey.com/forum/viewtopic.php?p=402574
//     fso.CreateFolder(Dest)
    
//     AppObj := ComObjCreate("Shell.Application")
//     FolderObj := AppObj.Namespace(Zip)
//     if Filename {
//         FileObj := FolderObj.ParseName(Filename)
//         AppObj.Namespace(Dest).CopyHere(FileObj, 4|16)
//     }
//     else
//     {
//         FolderItemsObj := FolderObj.Items()
//         AppObj.Namespace(Dest).CopyHere(FolderItemsObj, 4|16)
//     }
// }

