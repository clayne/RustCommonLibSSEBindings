
use std::os::windows::raw::HANDLE;
use commonlibsse_sys::root::REL::Version;
#[test]
fn simpleTest() {
    unsafe {
        let x = (commonlibsse_sys::root::SKSE::WinAPI::GetCurrentProcess()).cast::<HANDLE>();
        println!("{:?}",x);
    }
    println!("Simple Test Success");
}
#[test]
fn simpleTest2() {
    unsafe {
        let x = (commonlibsse_sys::root::SKSE::WinAPI::GetCurrentModule()).cast::<HANDLE>();
        println!("{:?}",x);
    }
}
