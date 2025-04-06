use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use libc::{c_char, c_int};
use smallvec::SmallVec;

#[repr(C)]
pub struct Features {
    pub bn_list: *mut Builtin,
    pub bn_size: c_int,
    pub cd_list: *mut Conddef,
    pub cd_size: c_int,
    pub mf_list: *mut MathFunc,
    pub mf_size: c_int,
    pub pd_list: *mut Paramdef,
    pub pd_size: c_int,
    pub n_abstract: c_int,
}

unsafe impl Send for Features {}

#[repr(C)]
pub struct Module {
    _priv: (),
}

#[repr(C)]
pub struct Builtin {
    pub node: HashNode,
    pub handlerfunc: HandlerFunc,
    pub minargs: c_int,
    pub maxargs: c_int,
    pub funcid: c_int,
    pub optstr: *mut c_char,
    pub defopts: *mut c_char,
}

type HandlerFunc = Option<
    unsafe extern "C" fn(
        *mut c_char,
        *mut *mut c_char,
        *mut Options,
        libc::c_int,
    ) -> libc::c_int,
>;

#[repr(C)]
pub struct Options {
    pub ind: [libc::c_uchar; 128],
    pub args: *mut *mut c_char,
    pub argscount: c_int,
    pub argsalloc: c_int,
}

#[repr(C)]
pub struct Conddef {
    _priv: (),
}

#[repr(C)]
pub struct MathFunc {
    _priv: (),
}
#[repr(C)]
pub struct Paramdef {
    _priv: (),
}

#[repr(C)]
pub struct HashNode {
    pub next: *mut HashNode,
    pub nam: *mut c_char,
    pub flags: c_int,
}

extern "C" {
    pub fn featuresarray(m: *mut Module, f: *mut Features) -> *mut *mut c_char;

    pub fn handlefeatures(
        m: *mut Module,
        f: *mut Features,
        enables: *mut *mut c_int,
    ) -> c_int;

    pub fn setfeatureenables(
        m: *mut Module,
        f: *mut Features,
        e: *mut libc::c_int,
    ) -> c_int;

    pub fn getsparam(s: *mut c_char) -> *mut c_char;
}

/**
 * Get a scalar (string) parameter from zsh.
 *
 * Note that the 'static is a lie: you must not hold onto the returned CStr.
 */
pub unsafe fn get_string_param(name: &CStr) -> Option<&'static CStr> {
    // zsh requires a mutable c_char array for the name:
    let mut v: SmallVec<[_; 32]> = CString::from(name)
        .into_bytes_with_nul()
        .into_iter()
        .map(|b| b as i8)
        .collect();
    let v = unsafe { getsparam(v.as_mut_ptr()) };
    if v == null_mut() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(v) })
    }
}
