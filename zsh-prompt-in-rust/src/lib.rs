use std::ffi::{CStr, CString};
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::sync::Mutex;

use once_cell::sync::Lazy;

mod git;
mod prompt;
mod zsh;

/// … the zsh API annoyingly requires all strings to be mutable. So, this holds their
/// heap-allocations.
struct Strings(Vec<Vec<libc::c_char>>);

impl Strings {
    pub fn new() -> Strings {
        Strings(Vec::new())
    }

    pub fn add(&mut self, s: &CStr) -> *mut libc::c_char {
        let mut v: Vec<_> = CString::from(s)
            .into_bytes_with_nul()
            .into_iter()
            .map(|b| b as i8)
            .collect();
        let p = v.as_mut_ptr();
        self.0.push(v);
        p
    }
}

struct OurModuleData {
    // The point of `builtins_table` and `strings` is to keep data alive on the heap. Pointers to
    // the data are passed to `zsh`, so they *look* unused to Rust.
    #[allow(dead_code)]
    builtins_table: Vec<zsh::Builtin>,
    features: zsh::Features,
    #[allow(dead_code)]
    strings: Strings,
}

impl OurModuleData {
    fn init() -> Box<OurModuleData> {
        let mut builtins_table = Vec::new();

        let mut strings = Strings::new();
        let builtin_name = strings.add(c"_rust-prompt-alpha");

        builtins_table.push(zsh::Builtin {
            node: zsh::HashNode {
                next: null_mut(),
                nam: builtin_name,
                flags: 0,
            },
            handlerfunc: Some(rust_prompt),
            minargs: 0,
            maxargs: -1,
            funcid: 0,
            optstr: null_mut(),
            defopts: null_mut(),
        });

        let features = zsh::Features {
            bn_list: builtins_table.as_mut_ptr(),
            bn_size: 1,
            cd_list: null_mut(),
            cd_size: 0,
            mf_list: null_mut(),
            mf_size: 0,
            pd_list: null_mut(),
            pd_size: 0,
            n_abstract: 0,
        };

        Box::new(OurModuleData {
            builtins_table,
            features,
            strings,
        })
    }

    unsafe fn features(&mut self) -> *mut zsh::Features {
        &mut self.features as *mut _
    }
}

unsafe impl Send for OurModuleData {}

unsafe extern "C" fn rust_prompt(
    _builtin_name: *mut libc::c_char,
    args: *mut *mut libc::c_char,
    opts: *mut zsh::Options,
    q: libc::c_int,
) -> libc::c_int {
    let _ = (opts, q);
    let args_v = unsafe { args_array_to_vec(&args) };
    let result = std::panic::catch_unwind(|| {
        prompt::prompt(&args_v)
    });
    match result {
        Ok(Ok(())) => 0,
        Ok(Err(err)) => err,
        Err(_) => {
            eprintln!("prompt panic!().");
            -42
        }
    }
}

unsafe fn args_array_to_vec<'a>(args: &'a *mut *mut libc::c_char) -> Vec<&'a CStr> {
    let mut args_vec = Vec::new();
    let mut args_iter = *args;
    loop {
        let arg = *args_iter;
        if arg == null_mut() {
            break;
        }
        let arg = unsafe { CStr::from_ptr(arg) };
        args_vec.push(arg);
        args_iter = args_iter.offset(1);
    }
    args_vec
}

static MODULE_DATA: Lazy<Mutex<Option<Box<OurModuleData>>>> = Lazy::new(|| Mutex::new(None));

#[unsafe(no_mangle)]
fn setup_(_m: *mut zsh::Module) -> libc::c_int {
    let mut mod_lock = MODULE_DATA.lock().unwrap();
    *mod_lock = Some(OurModuleData::init());
    0
}

#[unsafe(no_mangle)]
fn features_(m: *mut zsh::Module, features: *mut *mut *mut libc::c_char) -> libc::c_int {
    let mut mod_lock = MODULE_DATA.lock().unwrap();
    let module = mod_lock.deref_mut().as_mut().unwrap();
    let module_features = unsafe { module.features() };
    unsafe {
        *features = zsh::featuresarray(m, module_features);
    }
    0
}

#[unsafe(no_mangle)]
fn enables_(m: *mut zsh::Module, enables: *mut *mut libc::c_int) -> libc::c_int {
    let mut mod_lock = MODULE_DATA.lock().unwrap();
    let module = mod_lock.deref_mut().as_mut().unwrap();
    let module_features = unsafe { module.features() };
    unsafe { zsh::handlefeatures(m, module_features, enables) }
}

#[unsafe(no_mangle)]
fn boot_(_m: *mut zsh::Module) -> libc::c_int {
    0
}

#[unsafe(no_mangle)]
fn cleanup_(m: *mut zsh::Module) -> libc::c_int {
    let mut mod_lock = MODULE_DATA.lock().unwrap();
    let module = mod_lock.deref_mut().as_mut().unwrap();
    let module_features = unsafe { module.features() };
    unsafe { zsh::setfeatureenables(m, module_features, null_mut()) }
}

#[unsafe(no_mangle)]
fn finish_(_m: *mut zsh::Module) -> libc::c_int {
    let mut mod_lock = MODULE_DATA.lock().unwrap();
    *mod_lock = None;
    0
}
