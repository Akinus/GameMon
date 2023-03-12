// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 26 Feb 2023 @ 9:40:48                           #
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

#[cfg(windows)]
fn main() {
    
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
            }
        },
        ak_run::{activate,
            close_all_ahk,
            run_cmd, power_monitors
        },
        ak_utils::{
            macros::{
                exit_app,
                log
            },
            Cleanup,
            HKEY,
            Message,
            sleep,
            dark_hours
        },
    };
    use std::{path::{PathBuf}, sync::mpsc, panic, time::{Instant, Duration}};
    use tray_item::TrayItem;
    use winapi::um::{winuser::{GetDesktopWindow}};
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    use active_win_pos_rs::get_active_window;

    // Initialize Setup
    reg_check(HKEY);
    initialize_log(HKEY);
    let _cleanup = Cleanup;
    let (tx, rx) = mpsc::channel();
    let (filter_tx, filter_rx) = mpsc::channel();
    let (confirm_tx, confirm_rx) = mpsc::channel();
    let (refresh_tx, refresh_rx) = mpsc::channel();
    panic::set_hook(Box::new(|panic_info| {
        // Handle the panic here
        exit_app!(1, format!("{:?}", panic_info));
    }));

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

                for (sec, section) in &enum_keys {
                    exe_check.push(section.exe_name.clone());
                    if section.game_or_win == "Game" {game_check.push((section.exe_name.clone(), sec.clone()))};
                }

                let mut loop_window = get_active_window().unwrap().process_name;
                let mut current_window = unsafe { format!("{:?}", GetDesktopWindow()) };

                let mut current_profile = gamemon_value(HKEY, "current_profile").to_owned();
                
                let mut current_priority;
                
                let mut f_keys;

                let mut running = is_any_process_running(&game_check);
                let mut change = ftx.send((current_profile.clone(), get_section(current_profile.clone())));

                let mut start_times = vec![
                    Instant::now(),
                    Instant::now()
                ];

                let mut timers;

                loop {
                    sleep(250);
                    timers = vec![
                        start_times[0].elapsed(),
                        start_times[1].elapsed()
                    ];
                    
                    match change.clone() {
                        Ok(_) => (),
                        Err(send_error) => {
                            let _ = msg_box("", &send_error.0.0, 1500);
                        }
                    }

                    match refresh_rx.try_recv(){
                        Ok(_) => {
                            enum_keys.clear();
                            exe_check.clear();
                            game_check.clear();
                            
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

                            for (sec, section) in &enum_keys {
                                exe_check.push(section.exe_name.clone());
                                if section.game_or_win == "Game" {game_check.push((section.exe_name.clone(), sec.clone()))};
                            }
                        },
                        _ => ()
                    };

                    current_profile = gamemon_value(HKEY, "current_profile").to_owned();

                    if user_idle() {
                        if &current_profile == "Idle" {
                            if dark_hours(){
                                if timers[0] > Duration::from_secs(8){
                                    power_monitors(false);
                                    start_times[0] = Instant::now();
                                }

                                if gamemon_value(HKEY, "display") == "on"{
                                    let idle = get_idle();
                                    change = ftx.send(("Idle".to_string(), idle));
                                    confirm_rx.recv().unwrap();
                                }
                            } else {
                                if timers[0] > Duration::from_secs(8){
                                    power_monitors(true);
                                    start_times[0] = Instant::now();
                                }

                                if gamemon_value(HKEY, "display") == "off" {
                                    let idle = get_idle();
                                    change = ftx.send(("Idle".to_string(), idle));
                                    confirm_rx.recv().unwrap();
                                }
                            }
                            continue;
                        } else if get_value(HKEY, &current_profile, "game_or_win") != "Game" {
                            let idle = get_idle();
                            change = ftx.send(("Idle".to_string(), idle));
                            confirm_rx.recv().unwrap();
                        }
                        continue;
                    }

                    if timers[1] > Duration::from_secs(4){
                        running = is_any_process_running(&game_check);
                        start_times[1] = Instant::now();
                    }
                    
                    current_priority = gamemon_value(HKEY, "current_priority").to_owned();

                    if running.0 {
                        if running.1 != current_profile {
                            f_keys = filtered_keys(&mut enum_keys, &current_priority);
                            if f_keys.len() > 0 {
                                let sec = &f_keys.first().unwrap().0;
                                let section = f_keys.first().unwrap().1.clone();
                                change = ftx.send((sec.to_string(), section));
                                confirm_rx.recv().unwrap();
                            } else {
                                if &current_profile != "General" {
                                    let general = get_section("General");
                                    change = ftx.send(("General".to_owned(), general));
                                    confirm_rx.recv().unwrap();
                                }
                            };
                        }
                        continue;
                    }

                    if current_window != loop_window {
                        loop_window = current_window.clone();
                        f_keys = filtered_keys(&mut enum_keys, &current_priority);
                        if f_keys.len() > 0 {
                            let sec = &f_keys.first().unwrap().0;
                            let section = f_keys.first().unwrap().1.clone();
                            change = ftx.send((sec.to_string(), section));
                            confirm_rx.recv().unwrap();
                        } else {
                            if &current_profile != "General" {
                                let general = get_section("General");
                                change = ftx.send(("General".to_owned(), general));
                                confirm_rx.recv().unwrap();
                            }
                        };
                    }
                    current_window = match get_active_window(){
                        Ok(window) => window.process_name,
                        Err(_) => loop_window.clone()
                    };
                };
            });
            let ctx = confirm_tx.clone();
            std::thread::spawn(move ||{
                loop {
                    match filter_rx.recv() {
                        Ok((sec, section)) => {
                            activate((sec, section));
                            ctx.send(1).unwrap();
                            while let Ok(_) = filter_rx.try_recv() {};
                        },
                        _ => ()
                    };
                };
            });
            let mut count = 0;
            loop {
                // main_check();
                match rx.try_recv() {
                    Ok(Message::Quit) => {
                        exit_app!(1, "Menu");
                    }
                    Ok(Message::Gui) => {
                            main_gui();
                            refresh_tx.send(1).unwrap();
                    }
                    Ok(Message::Defaults) => {
                            defaults_gui();
                            refresh_tx.send(1).unwrap();
                    }
                    Ok(Message::Logs) => {
                        let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
                    }
                    Err(_) => (),
                };
                
                if count > 59 {
                    refresh_tx.send(1).unwrap();
                    count = 0;
                }
                count += 1;
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
}
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