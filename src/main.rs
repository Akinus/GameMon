// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 11 Feb 2023 @ 14:46:01                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

#![allow(non_snake_case)]
#![cfg_attr(
    all(
      target_os = "windows",
    //   not(feature = "console"),
    ),
    windows_subsystem = "windows",
  )]
//   Import Data ####
// extern crate winreg;
use {tray_item::TrayItem
    , winsafe::{prelude::*}
    , winsafe::{HWND, co::{MB}}
};
use ak_gui::windows::msg_box;
use crossbeam::{
    thread::scope,
    channel
};

mod ak_io;
mod ak_utils;
mod ak_gui;
mod ak_run;
use {
    ak_run::{
        close_all_ahk,
        run_cmd,
        activate,
        deactivate
    }
    , ak_io::{
        write::
            {
                write_key,
                reset_running,
            },
        read::
            {
                reg_check,
                gamemon_value,
                get_section,
                user_idle,
                get_idle,
                get_value,
                filtered_keys
            },
        logging::initialize_log
    }
    , ak_utils::{
        Cleanup,
        sleep,
        macros::
            {
                exit_app,
                log
            },
        Message,
        HKEY
    }
    , ak_gui::windows::{
        main_gui,
        defaults_gui
    }
};


#[cfg(windows)]
fn main() {
    // Initialize Setup

    use std::borrow::Borrow;
    reg_check(HKEY);
    initialize_log(HKEY);
    reset_running(HKEY);
    let _cleanup = Cleanup;

    // Create system tray
    let (tx, rx) = channel::bounded(2);
    let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

    tray.add_label("GameMon").unwrap();
    
    tray.add_menu_item("About", || {
        let hwnd = HWND::GetDesktopWindow();
        hwnd.MessageBox("GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language"
        , "About", 
        MB::OK | MB::ICONINFORMATION).unwrap();
    })
    .unwrap();
    
    let txc = tx.clone();
    
    tray.add_menu_item("View Logs", move || {
        println!("Logs");
        txc.send(Message::Logs).unwrap();
    }).unwrap();

    let txc = tx.clone();
    
    tray.add_menu_item("Monitors", move || {
        println!("GUI");
        txc.send(Message::Gui).unwrap();
    }).unwrap();
    
    let txc = tx.clone();

    
    tray.add_menu_item("Default Settings", move || {
        println!("Default Settings");
        txc.send(Message::Defaults).unwrap();
    }).unwrap();
    
    let txc = tx.clone();


    tray.add_menu_item("Quit", move || {
        println!("Quit");
        for _ in 0..10 {
            txc.send(Message::Quit).unwrap();
        }
    })
    .unwrap();

    let (exit_tx, exit_rx) = channel::bounded(2);
    // let mut count = 30;
    let mut keys = filtered_keys();
    let mut new_keys = Vec::new();
    let mut idle_time = get_value(HKEY, "Idle", "exe_name").parse::<u64>().unwrap();
    let mut c = gamemon_value(HKEY, "current_profile").to_owned();

    'main: loop {

        if user_idle(idle_time) {
            if &c != &"Idle"{
                deactivate((&c, get_section(&c)));
                activate(("Idle", get_idle()));
                keys = filtered_keys();
                c = gamemon_value(HKEY, "current_profile").to_owned();
            }
            sleep(2000);
            continue;
        }

        new_keys = keys.clone();
        if !new_keys.is_empty(){
            for entry in new_keys {
                let t = entry.clone();
                if &t.0 != &c {
                    deactivate((&c, get_section(&c)));
                    activate(t);
                    keys = filtered_keys();
                    c = gamemon_value(HKEY, "current_profile").to_owned();
                }
            }
        } else {
            if c != "General".to_string() {
                deactivate((&c, get_section(&c)));
                activate(("General", get_section("General")));
                keys = filtered_keys();
                c = gamemon_value(HKEY, "current_profile").to_owned();
            }
        }

        match rx.try_recv() {
            Ok(Message::Quit) => {
                exit_tx.send(1).unwrap();
                break 'main;
            },
            Ok(Message::Gui) => {
                scope(|s| {
                    let t = s.spawn(|_| main_gui());
                    t.join().unwrap();
                }).unwrap();
                idle_time = get_value(HKEY, "Idle", "exe_name").parse::<u64>().unwrap();
            },
            Ok(Message::Defaults) => {
                scope(|s| {
                    s.spawn(|_| defaults_gui());
                }).unwrap();
            },
            Ok(Message::Logs) => {
                let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
            },
            Err(_) => ()
        };
        // count += 1;
        sleep(250);
    }

    match exit_rx.recv() {
        Ok(0) => {
            exit_app!(0, "Memory allocation too high!!");
        },
        Ok(1) => {
            exit_app!(1, "Menu");
        },
        Ok(_) => (),
        Err(_) => ()
    }
        
}
                
                

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

//************************************************************ */
// *********************** TESTS ****************************
//************************************************************ */
#[cfg(test)]
use std::ffi::OsString;
use std::io::Error;
use std::mem;
use std::ptr;

use winapi::shared::minwindef::{LPARAM, TRUE, BOOL};
use winapi::shared::windef::{HMONITOR, HDC, LPRECT};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};

#[test]

fn enumerate_monitors() {
    // Define the vector where we will store the result
    let mut monitors = Vec::<MONITORINFOEXW>::new();
    let userdata = &mut monitors as *mut _;

    let result = unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(enum_monitor_callback),
            userdata as LPARAM,
        )
    };

    if result != TRUE {
        // Get the last error for the current thread.
        // This is analogous to calling the Win32 API GetLastError.
        panic!("Could not enumerate monitors: {}", Error::last_os_error());
    }

    for m in monitors{
        let r = msg_box("", format!("{:?}\n{}", m.rcMonitor, m.dwFlags) , 1500);
    }
}

unsafe extern "system" fn enum_monitor_callback(
    monitor: HMONITOR,
    _: HDC,
    _: LPRECT,
    userdata: LPARAM,
) -> BOOL {
    // Get the userdata where we will store the result
    let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);

    // Initialize the MONITORINFOEXW structure and get a pointer to it
    let mut monitor_info: MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    // Call the GetMonitorInfoW win32 API
    let result = GetMonitorInfoW(monitor, monitor_info_ptr);
    if result == TRUE {
        // Push the information we received to userdata
        monitors.push(monitor_info);
    }

    TRUE
}