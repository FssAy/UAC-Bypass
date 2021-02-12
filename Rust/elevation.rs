use std::ptr::null_mut;
use std::{mem, process};

use libc;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{FALSE, TRUE};

use winapi::um::securitybaseapi::{
    CheckTokenMembership,
    AllocateAndInitializeSid,
    FreeSid,
};

use winapi::um::winnt::{
    SECURITY_NT_AUTHORITY,
    SID_IDENTIFIER_AUTHORITY,
    SECURITY_BUILTIN_DOMAIN_RID,
    DOMAIN_ALIAS_RID_ADMINS,
    PSID,
};


pub unsafe fn is_elevated() -> bool {
    let mut authority: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY::default();
    authority.Value = SECURITY_NT_AUTHORITY;

    let mem = libc::malloc(mem::size_of::<PSID>()) as *mut c_void;
    let mut admins_group: PSID = *(mem as *mut PSID);

    let mut sid_result = AllocateAndInitializeSid(
        &mut authority,
        2,
        SECURITY_BUILTIN_DOMAIN_RID,
        DOMAIN_ALIAS_RID_ADMINS,
        0, 0, 0, 0, 0, 0,
        &mut admins_group
    );

    if sid_result == TRUE {
        if CheckTokenMembership(null_mut(), admins_group, &mut sid_result) == FALSE {
            sid_result = FALSE
        }

        FreeSid(admins_group);
    }

    return if sid_result == TRUE {
        true
    } else {
        false
    }
}
