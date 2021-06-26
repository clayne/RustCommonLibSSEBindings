#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn testing() {
    unsafe {
        use root::SKSE::WinAPI::GetCurrentProcess;
        let x = GetCurrentProcess();
    }
}
