use std::{ptr, net::ToSocketAddrs};
use std::ffi::CStr;
use uuid::{Uuid, Version};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, TH32CS_SNAPPROCESS, PROCESSENTRY32};

use std::net::SocketAddr;
use winapi::um::winsock2;
use winapi::shared::ws2def;

fn main() {

    // get machine id
    let machine_id = Uuid::new_v4(); // create a new instance of Uuid
    assert_eq!(Some(Version::Random), machine_id.get_version()); //ensure that the expected id and the actual id match
    println!("Machine ID: {}", machine_id);

    // get ip address
    const BUFFER_SIZE: usize = ws2def::NI_MAXHOST as usize;
    let mut buffer = [0u8; BUFFER_SIZE];

    let result = unsafe {
        winsock2::gethostname(buffer.as_mut_ptr() as *mut i8, BUFFER_SIZE as i32)
    };

    if result != 0 {
        return println!("Error: Unable to get host name.");
    }

    let hostname = match String::from_utf8(buffer.to_vec()) {
        Ok(s) => s,
        Err(_) => {
            return println!("Error: Host name is not valid UTF-8.");
        }
    };

    let local_ips = match hostname.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(_) => {
            return println!("Error: Unable to resolve host name.");
        }
    };

    let local_ip = local_ips.filter(|addr| match addr {
        SocketAddr::V4(v4) => !v4.ip().is_loopback(),
        SocketAddr::V6(v6) => !v6.ip().is_loopback(),
    })
    .next();

    match local_ip {
        Some(ip) => println!("Local IP: {}", ip),
        None => println!("Error: No local IP address found."),
    }

    // get user device info using whoami crate
    println!("User's Name: {}", whoami::realname(), );
    println!("User's Username: {}", whoami::username(), );
    println!("User's Language: {:?}", whoami::lang().collect::<Vec<String>>(), );
    println!("Device's Pretty Name: {}", whoami::devicename(), );
    println!("Device's Hostname: {}", whoami::hostname(), );
    println!("Device's Platform: {}", whoami::platform(), );
    println!("Device's OS Distro: {}", whoami::distro(), );
    println!("Device's Desktop Env: {}", whoami::desktop_env(), );
    println!("Device's CPU Arch: {}", whoami::arch(), );
    
    // get active browser lists from process
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
