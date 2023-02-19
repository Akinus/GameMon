// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 19 Feb 2023 @ 13:28:33                          #
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
mod ak_gui;
mod ak_run;
mod ak_io;
mod ak_utils;
  
use crate::{ak_gui::windows::
    {
        msg_box,
        main_gui,
        defaults_gui
    },
    ak_io::{logging::initialize_log,
        read::{filtered_keys,
            gamemon_value,
            get_idle,
            get_section,
            get_value,
            Instance,
            is_any_process_running,
            reg_check,
            user_idle
        },
        write::{write_key}
    },
    ak_run::{activate,
        close_all_ahk,
        run_cmd
    },
    ak_utils::{
        macros::{
            exit_app,
            log
        },
        Cleanup,
        HKEY,
        Message,
        sleep
    },
};
use std::{path::{PathBuf}, sync::mpsc};
use tray_item::TrayItem;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use wintrap;
use active_win_pos_rs::get_active_window;

#[cfg(windows)]
fn main() {
    // Initialize Setup
    reg_check(HKEY);
    initialize_log(HKEY);
    let _cleanup = Cleanup;
    let (tx, rx) = mpsc::channel();
    let (filter_tx, filter_rx) = mpsc::channel();

    wintrap::trap(
        &[
            wintrap::Signal::CloseWindow,
            wintrap::Signal::CloseConsole,
            wintrap::Signal::CtrlC,
            wintrap::Signal::CtrlBreak
        ],
        move |signal| {
            // handle signal here
            // let _ = msg_box("", format!("Caught a signal: {:?}", signal), 1500);
            match signal {
                wintrap::Signal::CloseWindow => {
                    exit_app!(1, "Window Messsage: WM_CLOSE");
                }
                wintrap::Signal::CloseConsole => {
                    exit_app!(1, "Window Messsage: WM_QUIT");
                }
                wintrap::Signal::CtrlC => {
                    exit_app!(1, "Window Messsage: WM_DESTROY");
                }
                wintrap::Signal::CtrlBreak => {
                    exit_app!(1, "Window Messsage: WM_ENDSESSION");
                }
            }
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

                let mut current_profile;
                
                let mut f_keys;

                let mut count = 0;
                let mut running = is_any_process_running(&game_check);
                let mut change = Ok(());
    
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

                    current_profile = gamemon_value(HKEY, "current_profile").to_owned();
                    
                    if user_idle() {
                        if &current_profile == "Idle" {
                            continue;
                        } else if get_value(HKEY, &current_profile, "game_or_win") != "Game" {
                            let idle = get_idle();
                            change = ftx.send(("Idle".to_string(), idle));
                        }
                        continue;
                    } else if &current_profile == "Idle" {
                        let general = get_section("General");
                        change = ftx.send(("General".to_owned(), general));
                    }

                    if count > 4 {
                        running = is_any_process_running(&game_check);
                    }

                    current_window = get_active_window().unwrap().process_name;

                    if running {
                        f_keys = filtered_keys(&mut enum_keys, &current_profile);
                        if f_keys.len() > 0 {
                            let sec = &f_keys.first().unwrap().0;
                            let section = f_keys.first().unwrap().1.clone();
                            change = ftx.send((sec.to_string(), section));
                        } else {
                            if &current_profile != "General" {
                                let general = get_section("General");
                                change = ftx.send(("General".to_owned(), general));
                            }
                        };
                        continue;
                    }

                    if current_window != loop_window {
                        loop_window = current_window;
                        f_keys = filtered_keys(&mut enum_keys, &current_profile);
                        if f_keys.len() > 0 {
                            let sec = &f_keys.first().unwrap().0;
                            let section = f_keys.first().unwrap().1.clone();
                            change = ftx.send((sec.to_string(), section));
                        } else {
                            if &current_profile != "General" {
                                let general = get_section("General");
                                change = ftx.send(("General".to_owned(), general));
                            }
                        };
                        
                    }

                    match change.clone() {
                        Ok(_) => (),
                        Err(send_error) => {
                            let _ = msg_box("", &send_error.0.0, 1500);
                        }
                    }
                    
                    sleep(250);
                    count += 1;
                };
            });

            let frx = filter_rx;

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
                        exit_app!(1, "Menu");
                    }
                    Ok(Message::Gui) => {
                            let edit = std::thread::spawn(move || main_gui());
                            edit.join().unwrap();
                            
                    }
                    Ok(Message::Defaults) => {
                            std::thread::spawn(move || defaults_gui());
                    }
                    Ok(Message::Logs) => {
                        let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
                    }
                    Err(_) => (),
                };
                
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
// #[test]
// fn test(){
//     use winsafe;
//     use sysinfo::{Pid, PidExt, System, SystemExt, Process, ProcessExt};
//     let process_names: HashSet<String> = ["notepad.exe", "calc.exe"].iter().map(|s| s.to_string()).collect();

//     loop {
//         if window_is_active("Code.exe"){
//             let _ = msg_box("", "TRUE", 500);
//         } else {
//             continue;
//         }
        
//         // std::thread::sleep(std::time::Duration::from_millis(500));
//     }
// }