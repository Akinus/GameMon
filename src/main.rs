// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Wed, 28 Dec 2022 @ 21:03:39                          #
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
use {std::{sync::mpsc, path::Path, cmp::Ordering}
    , tray_item::TrayItem
    , winsafe::{prelude::*}
    , winsafe::{HWND, co::{MB}}
};
use winreg::{RegKey, enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE}};
use sysinfo::{System, SystemExt, ProcessExt};
mod ak_io;
mod ak_utils;
mod ak_gui;
mod ak_run;
use {
    ak_run::{
        close_all_ahk,
        run_cmd,
        main_check
    }
    , ak_io::{
        write::
            {
                write_key,
                reset_running,
                reg_write_value,
            },
        read::
            {
                reg_check,
            },
        logging::initialize_log
    }
    , ak_utils::{
        Cleanup,
        macros::
            {
                exit_app,
                log
            },
        Message
    }
    , ak_gui::windows::{
        main_gui,
        defaults_gui
    }
};


#[cfg(windows)]
fn main() {
    // Initialize Setup


    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    reg_check(&hklm);
    initialize_log(&hklm);
    reset_running(&hklm);
    let _cleanup = Cleanup;

    // Create system tray
    let (tx, rx) = mpsc::channel();
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
        txc.send(Message::Quit).unwrap();
    })
    .unwrap();

    let _v = reg_write_value(&hklm, &Path::new("Software").join("GameMon")
        , "current_profile".to_string()
        , "General".to_string());

    let mut system = System::new_all();
    
    loop {

        system.refresh_all();

        if system.processes_by_exact_name("GameMon.exe").last().unwrap().memory()
            .cmp(&"1073741824".parse::<u64>().unwrap()) == Ordering::Greater {
                exit_app!(0, "Memory allocation too high");
        };
        
        'channel: loop {
            match rx.try_recv(){
                Ok(Message::Quit) => exit_app!(1, "Menu"),
                Ok(Message::Gui) => {
                    let t = std::thread::spawn(|| {
                        main_gui();
                    });
                    t.join().unwrap();
                },
                Ok(Message::Defaults) => {
                    let t = std::thread::spawn(||{
                        defaults_gui();
                    });
                    t.join().unwrap();
                },
                Ok(Message::Logs) => {
                    let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
                },
                Err(_) => break 'channel,
            };
        }

        main_check(&system);
        
    } // End Loop
    
}
                
                

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
