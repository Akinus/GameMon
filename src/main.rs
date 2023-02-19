// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 19 Feb 2023 @ 9:09:12                           #
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
use active_win_pos_rs::ActiveWindow;
//   Import Data ####
// extern crate winreg;
use crossbeam::{channel, thread::scope};
use sysinfo::{SystemExt, RefreshKind, ProcessRefreshKind, ProcessExt, Process};
use winsafe::msg::wm::GetText;
use {
    tray_item::TrayItem,
    winsafe::prelude::*,
    winsafe::{co::MB, HWND},
};

use crate::{ak_gui::windows::msg_box, ak_io::read::window_is_active};

mod ak_gui;
mod ak_io;
mod ak_run;
mod ak_utils;
use {
    ak_gui::windows::{defaults_gui, main_gui},
    ak_io::{
        logging::initialize_log,
        read::reg_check,
        write::{reset_running, write_key},
    },
    ak_run::{close_all_ahk, run_cmd},
    ak_utils::{
        macros::{exit_app, log},
        sleep, Cleanup, Message, HKEY,
    },
};
use crate::{
    ak_io::read::{filtered_keys, get_idle, get_section, get_value, user_idle, Instance},
    ak_run::{activate, deactivate},
};
use ak_io::read::{gamemon_value, get_pid, get_cmd_line};
use crossbeam::channel::Receiver;
use std::{path::{Path, PathBuf}, collections::HashSet, ptr};
use std::ptr::null_mut;
use winapi::{um::{winuser::{
    DispatchMessageA, PeekMessageA, TranslateMessage, MSG, WM_CLOSE, WM_DESTROY, WM_ENDSESSION,
    WM_QUIT, GetActiveWindow, GetWindowThreadProcessId,
}, processthreadsapi::GetProcessId}, shared::minwindef::DWORD};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use wintrap;

#[cfg(windows)]
fn main() {
    // Initialize Setup

    use std::{rc::Rc, sync::{Arc, Mutex}};

    use active_win_pos_rs::get_active_window;

    use crate::ak_io::read::is_any_process_running;
    reg_check(HKEY);
    initialize_log(HKEY);
    let _cleanup = Cleanup;
    let (tx, rx) = channel::bounded(2);
    let (exit_tx, exit_rx) = channel::bounded(2);
    let (filter_tx, filter_rx) = channel::bounded(2);
    let bexit_tx = exit_tx.clone();

    wintrap::trap(
        &[wintrap::Signal::CloseWindow],
        move |_signal| {
            // handle signal here
            // let _ = msg_box("", format!("Caught a signal: {:?}", signal), 1500);
            bexit_tx.send(WM_CLOSE).unwrap();
        },
        || {
            // do work
            
            // Create system tray
            let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

            tray.add_label("GameMon").unwrap();

            tray.add_menu_item("About", || {
                let _ = msg_box(
                    "About GameMon.exe",
                    "GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language",
                    0,
                );
            })
            .unwrap();

            let txc = tx.clone();

            tray.add_menu_item("View Logs", move || {
                println!("Logs");
                txc.send(Message::Logs).unwrap();
            })
            .unwrap();

            let txc = tx.clone();

            tray.add_menu_item("Monitor Settings", move || {
                println!("GUI");
                txc.send(Message::Gui).unwrap();
            })
            .unwrap();

            let txc = tx.clone();

            tray.add_menu_item("Default Settings", move || {
                println!("Default Settings");
                txc.send(Message::Defaults).unwrap();
            })
            .unwrap();

            let txc = tx.clone();

            tray.add_menu_item("Quit", move || {
                println!("Quit");
                for _ in 0..10 {
                    txc.send(Message::Quit).unwrap();
                }
            })
            .unwrap();

            let ftx = filter_tx.clone();

            std::thread::spawn(move || {
                let mut enum_keys;
                enum_keys = RegKey::predef(HKEY_LOCAL_MACHINE)
                    .open_subkey(&PathBuf::from("Software").join("GameMon"))
                    .unwrap()
                    .enum_keys()
                    .map(|x| {
                        let y = x.unwrap().clone();
                        let z = y.clone();
                        (z, get_section(y))
                    })
                    .filter(|entry| {
                        entry.0 != "Idle".to_string() && entry.0 != "General" && entry.0 != "defaults"
                    })
                    .collect::<Vec<(String, Instance)>>();

                let mut exe_check = Vec::new();
                let mut game_check = Vec::new();

                for (_, section) in &enum_keys {
                    exe_check.push(section.exe_name.clone());
                    if section.game_or_win == "Game" {game_check.push(section.exe_name.clone())};
                }

                let mut loop_window = get_active_window().unwrap().process_name;
                let mut current_window;

                let mut current_profile = gamemon_value(HKEY, "current_profile").to_owned();
                
                let mut current_priority = gamemon_value(HKEY, "current_priority").to_owned();
                let mut f_keys;
                f_keys = filtered_keys(&mut enum_keys, &current_profile, &current_priority);

                let mut count = 0;
                let mut running = is_any_process_running(&game_check);
    
                loop {

                    if count > 500 {
                        enum_keys = RegKey::predef(HKEY_LOCAL_MACHINE)
                            .open_subkey(&PathBuf::from("Software").join("GameMon"))
                            .unwrap()
                            .enum_keys()
                            .map(|x| {
                                let y = x.unwrap().clone();
                                let z = y.clone();
                                (z, get_section(y))
                            })
                            .filter(|entry| {
                                entry.0 != "Idle".to_string() && entry.0 != "General" && entry.0 != "defaults"
                            })
                            .collect::<Vec<(String, Instance)>>();

                        for (_, section) in &enum_keys {
                            exe_check.push(section.exe_name.clone());
                            if section.game_or_win == "Game" {game_check.push(section.exe_name.clone())};
                        }
                    }
                    
                    if user_idle() {
                        if &current_profile != "Idle"
                            && get_value(HKEY, &current_profile, "game_or_win") != "Game"
                        {
                            let idle = get_idle();
                            ftx.send(("Idle".to_string(), idle)).unwrap();
                        }
                        continue;
                    } else if &current_profile == "Idle" {
                        let general = get_section("General");
                        ftx.send(("General".to_owned(), general)).unwrap();
                    }

                    if count > 4 {
                        running = is_any_process_running(&game_check);
                        current_priority = gamemon_value(HKEY, "current_priority").to_owned();
                        current_profile = gamemon_value(HKEY, "current_profile").to_owned();
                    }

                    current_window = get_active_window().unwrap().process_name;

                    if running {
                        f_keys = filtered_keys(&mut enum_keys, &current_profile, &current_priority);
                        if f_keys.len() > 0 {
                            let sec = &f_keys.first().unwrap().0;
                            let section = f_keys.first().unwrap().1.clone();
                            ftx.send((sec.to_string(), section)).unwrap();
                        } else {
                            if &current_profile != "General" {
                                let general = get_section("General");
                                ftx.send(("General".to_owned(), general)).unwrap();
                            }
                        };
                        continue;
                    }

                    if current_window != loop_window {
                        loop_window = current_window;
                        f_keys = filtered_keys(&mut enum_keys, &current_profile, &current_priority);
                        if f_keys.len() > 0 {
                            let sec = &f_keys.first().unwrap().0;
                            let section = f_keys.first().unwrap().1.clone();
                            ftx.send((sec.to_string(), section)).unwrap();
                        } else {
                            if &current_profile != "General" {
                                let general = get_section("General");
                                ftx.send(("General".to_owned(), general)).unwrap();
                            }
                        };
                        
                    }
                    
                    sleep(250);
                    count += 1;
                };
            });

            let frx = filter_rx.clone();

            std::thread::spawn(move ||{
                loop {
                    match frx.recv() {
                        Ok((sec, section)) => {
                            activate((sec, section));
                        },
                        _ => ()
                    };
                };
            });

            loop {
                // main_check();
                match rx.try_recv() {
                    Ok(Message::Quit) => {
                        exit_tx.send(1).unwrap();
                    }
                    Ok(Message::Gui) => {
                        scope(|s| {
                            let edit = s.spawn(|_| main_gui());
                            edit.join().unwrap();
                        }).unwrap();
                            
                    }
                    Ok(Message::Defaults) => {
                        scope(|s| {
                            s.spawn(|_| defaults_gui());
                        })
                        .unwrap();
                    }
                    Ok(Message::Logs) => {
                        let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
                    }
                    Err(_) => (),
                };
                
                match exit_rx.try_recv() {
                    Ok(0) => {
                        exit_app!(0, "Memory allocation too high!!");
                    }
                    Ok(1) => {
                        exit_app!(1, "Menu");
                    }
                    Ok(WM_CLOSE) => {
                        exit_app!(1, "Window Messsage: WM_CLOSE");
                    }
                    Ok(WM_QUIT) => {
                        exit_app!(1, "Window Messsage: WM_QUIT");
                    }
                    Ok(WM_DESTROY) => {
                        exit_app!(1, "Window Messsage: WM_DESTROY");
                    }
                    Ok(WM_ENDSESSION) => {
                        exit_app!(1, "Window Messsage: WM_ENDSESSION");
                    }
                    Ok(_) => (),
                    Err(_) => (),
                }
                
                sleep(1000);
            }
        },
    )
    .unwrap();
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

//************************************************************ */
// *********************** TESTS ****************************
//************************************************************ */
#[test]
fn test(){
    use winsafe;
    use sysinfo::{Pid, PidExt, System, SystemExt, Process, ProcessExt};
    let process_names: HashSet<String> = ["notepad.exe", "calc.exe"].iter().map(|s| s.to_string()).collect();

    loop {
        if window_is_active("Code.exe"){
            let _ = msg_box("", "TRUE", 500);
        } else {
            continue;
        }
        
        // std::thread::sleep(std::time::Duration::from_millis(500));
    }
}