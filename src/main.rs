use std::ptr;
use std::ffi::CStr;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, TH32CS_SNAPPROCESS, PROCESSENTRY32};

fn main() {
    let snapshot = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    };
    if snapshot == ptr::null_mut() {
        return;
    }

    let mut process_entry: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    let success = unsafe {
        Process32First(snapshot, &mut process_entry)
    };
    if success == 0 {
        return;
    }

    loop {
        let c_str = unsafe { CStr::from_ptr(process_entry.szExeFile.as_ptr()) };
        let os_str = c_str.to_str();
        let process_name = os_str.unwrap();

        if process_name == "iexplore.exe" {
            println!("Internet Explorer is running");
        }
        if process_name == "firefox.exe" {
            println!("Firefox is running");
        }
        if process_name == "chrome.exe" {
            println!("Google Chrome is running");
        }
        if process_name == "brave.exe" {
            println!("Brave is running");
        }
        if process_name == "msedge.exe" {
            println!("Edge is running");
        }
        if process_name == "safari.exe" {
            println!("Safari is running");
        }
        if process_name == "opera.exe" {
            println!("Opera is running");
        }
        if process_name == "ucbrowser.exe" {
            println!("UC is running");
        }
        if process_name == "qqbrowser.exe" {
            println!("QQ is running");
        }
        if process_name == "baidubrowser.exe" {
            println!("Baidu is running");
        }
        
        let success = unsafe {Process32Next(snapshot, &mut process_entry)};
        if success == 0 {break;}
    }
}
