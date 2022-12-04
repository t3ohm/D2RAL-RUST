use std::{ffi::c_void};
use widestring::{U16CString, U16String};
use wincredentials_bindings::Windows::Win32::{System::SystemInformation::GetSystemTimeAsFileTime,
    Foundation::*, Security::Credentials::*
};

#[allow(unused_must_use)]
const NO_FLAGS: u32 = 0;


const GENERIC_CREDENTIAL: u32 = 1;
// const CRED_TYPE_DOMAIN_PASSWORD: u32 = 2;
// const CRED_TYPE_DOMAIN_CERTIFICATE: u32 = 3;
// const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: u32 = 4;
// const CRED_TYPE_GENERIC_CERTIFICATE: u32 = 5;
// const CRED_TYPE_DOMAIN_EXTENDED: u32 = 6;
// const CRED_TYPE_MAXIMUM: u32 = 7;

// const CRED_PERSIST_SESSIONL: u32 = 1;
const CRED_PERSIST_LOCAL_MACHINE: u32 = 2;
// const CRED_PERSIST_ENTERPRISE: u32 = 3;

#[derive(Debug, Clone)]
pub struct Credential {
    pub target: String,
    pub username: String,
    // UserName: PWSTR(user_ptr as *mut u16),
    pub secret: String,
    // CredentialBlobSize: secret_len as u32 * 2,
    // CredentialBlob: secret_ptr as *mut u8,
    pub comment: String,
    // Comment: PWSTR(comment_ptr as *mut u16),
    pub targetalias: String,
    // pub TargetName: String, //PWSTR(target_ptr as *mut u16),
    // pub TargetAlias: String, //PWSTR(std::ptr::null_mut() as *mut u16),
    // pub Flags: CRED_FLAGS(NO_FLAGS),
    // pub Type: CRED_TYPE(GENERIC_CREDENTIAL),
    // pub LastWritten: String, //unsafe { *filetime },
    // pub persist: u32,
    // pub attribute_count:u32,
    // pub Attributes: std::ptr::null_mut(),
}
impl PartialEq for Credential {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}
impl Eq for Credential {}
impl Default for Credential {
    fn default() -> Self {
        Self {
            target: Default::default(), 
            username: Default::default(), 
            secret: Default::default(),
            comment: Default::default(), 
            targetalias: Default::default() ,
            // persist: CRED_PERSIST_LOCAL_MACHINE,
            // attribute_count: 0,
        }
    }
}
// If the following Operations fail for any reason, ie; no credential,
// the result will resolve to an error.(CRASH)
// pub fn read_cred_session(target: &str) -> Credential {
//     read_credential(&target,GENERIC_CREDENTIAL,NO_FLAGS)
// }
pub fn read_cred_generic(target: &str) -> Credential {
    read_credential(&target,GENERIC_CREDENTIAL,NO_FLAGS)
}
// pub fn read_cred_enterprise(target: &str) -> Credential {
//     read_credential(&target,GENERIC_CREDENTIAL,NO_FLAGS)
// }
pub fn read_credential(target: &str,credtype:u32,flags:u32) -> Credential {
    let target_cstr = U16CString::from_str(target).unwrap();
    let target_ptr = target_cstr.as_ptr();
    // Allocate a pointer for the credential and read it
    let mut cred: *mut CREDENTIALW = std::ptr::null_mut();
    let cred_ptr: *mut *mut CREDENTIALW = &mut cred;
    unsafe {
        //CredReadW(PWSTR(target_ptr as *mut u16),GENERIC_CREDENTIAL,NO_FLAGS,cred_ptr)
        CredReadW(PWSTR(target_ptr as *mut u16),credtype,flags,cred_ptr)
    };
    // Read from the credential and convert it into something rustier
    let credential = unsafe {
        Credential {
            target: U16CString::from_ptr_str((*cred).TargetName.0 as *const u16).to_string_lossy(),
            username: U16CString::from_ptr_str((*cred).UserName.0 as *const u16).to_string_lossy(),
            // Divide U16String to number of elements, not the number of bytes
            secret: U16String::from_ptr(
                (*cred).CredentialBlob as *const u16,
                (*cred).CredentialBlobSize as usize / 2,
            ).to_string_lossy(),
            comment: {
                let mut comment:String = Default::default();
                if (*cred).Comment.0 != std::ptr::null_mut() {
                    comment = U16CString::from_ptr_str((*cred).Comment.0 as *const u16).to_string_lossy()
                }
                comment
            },
            targetalias: {
                let mut targetalias:String = Default::default();
                if (*cred).TargetAlias.0 != std::ptr::null_mut() {
                    targetalias = U16CString::from_ptr_str((*cred).TargetAlias.0 as *const u16).to_string_lossy()
                }
                targetalias
            },
            // persist: {
            //     let mut persist:u32 = Default::default();
            //     if (*cred).Persist.0 != 0 {
            //         persist = (*cred).Persist.0 as u32;
            //     }
            //     persist
            // },
            // attribute_count: {
            //     let mut attribute_count:u32 = Default::default();
            //     if (*cred).AttributeCount != 0 {
            //         attribute_count = (*cred).AttributeCount as u32;
            //     }
            //     attribute_count
            // },
            
        }
    };
    // Free the credential we read
    unsafe { CredFree(cred as *const c_void) };
    credential
}
pub fn write_credential(cred: Credential) {
    // Get the current time as a Windows file time
    let filetime = Box::new(FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    });
    let filetime: *mut FILETIME = Box::into_raw(filetime);
    unsafe { GetSystemTimeAsFileTime(filetime) };
    let secret_len = cred.secret.len();
    let target_cstr = U16CString::from_os_str(cred.target).unwrap();
    let secret_cstr = U16CString::from_os_str(cred.secret).unwrap();
    let user_cstr = U16CString::from_os_str(cred.username).unwrap();
    let targetalias_cstr = U16CString::from_os_str(cred.targetalias).unwrap();
    let comment_cstr = U16CString::from_os_str(cred.comment).unwrap();
    //let persist :u32 = cred.persist;
    //let persist :u32 = cred.persist;
    //let attribute_count:u32 = cred.attribute_count;
    let attribute_count:u32 = 0;
    let target_ptr = target_cstr.as_ptr();
    let secret_ptr = secret_cstr.as_ptr();
    let user_ptr = user_cstr.as_ptr();
    let comment_ptr = comment_cstr.as_ptr();
    let targetalias_ptr = targetalias_cstr.as_ptr();
    let cred = CREDENTIALW {
        TargetName: PWSTR(target_ptr as *mut u16),
        TargetAlias: PWSTR(targetalias_ptr as *mut u16),
        UserName: PWSTR(user_ptr as *mut u16),
        CredentialBlobSize: secret_len as u32 * 2,
        CredentialBlob: secret_ptr as *mut u8,
        Comment: PWSTR(comment_ptr as *mut u16),
        Flags: CRED_FLAGS(NO_FLAGS),
        Type: CRED_TYPE(GENERIC_CREDENTIAL),
        LastWritten: unsafe { *filetime },
        Persist: CRED_PERSIST(CRED_PERSIST_LOCAL_MACHINE),
        AttributeCount: attribute_count,
        Attributes: std::ptr::null_mut(),
    };
    unsafe { CredWriteW(&cred, NO_FLAGS) };
    // Free the file time object we got
    unsafe { drop(Box::from_raw(filetime)) }
}
pub fn delete_credential(target: &str) {
    let target_cstr = U16CString::from_str(target).unwrap();
    let target_ptr = target_cstr.as_ptr();
    unsafe { CredDeleteW(PWSTR(target_ptr as *mut u16), GENERIC_CREDENTIAL, NO_FLAGS) };
}