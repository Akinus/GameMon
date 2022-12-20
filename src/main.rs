// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Tue, 20 Dec 2022 @ 9:20:19                           #
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
use std::{path::{Path}, cmp::Ordering};
use {std::sync::mpsc, tray_item::TrayItem};
use winreg::{enums::*};
use winreg::RegKey;

use winsafe::{prelude::*};
use winsafe::{HWND, co::{MB}};

mod ak_io;
mod ak_utils;
mod ak_gui;
mod ak_run;
use ak_run::{
    close_all_ahk,
    run_cmd,
    change_open_rgb,
    change_signal_rgb,
    change_voice_attack,
    power_monitors,
    run_screensaver, 
    run_other_commands, 
    run_ahk, 
    close_ahk
};
use ak_io::{
    write::
        {
            write_key,
            reset_running,
            reg_write_value,
        },
    read::
        {
            get_section,
            ss_get,
            get_value,
            reg_check,
            user_idle,
            gamemon_value,
            process_exists,
            window_is_active
        },
    logging::initialize_log
};
use ak_utils::{
    sleep,
    Cleanup,
    macros::
        {
            exit_app,
            log
        },
    Message,
    dark_hours,
    memory_check
};
use ak_gui::windows::{
    main_gui,
    defaults_gui
};


#[cfg(windows)]
fn main() {
    // Initialize Setup
    reg_check();
    initialize_log();
    reset_running();
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

    let _v = reg_write_value(&Path::new("Software").join("GameMon")
        , "current_profile".to_string()
        , "General".to_string());
    
    loop {

        memory_check();
        
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

        
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon");
        
        for sec in hklm.open_subkey(path).unwrap().enum_keys().map(|x| x.unwrap()){

            let thread = std::thread::spawn(move || {
                let section = match sec.as_str() {
                    "defaults" => return,
                    "General" => return,
                    "Idle" => return,
                    _ => get_section(&sec),
                };

                // let defaults = get_defaults();
                let wait_time = get_value("Idle".to_string(), "exe_name".to_string());
                let night_srgb = get_value("defaults".to_string(), "night_hour_srgb_profile".to_string());
                let night_orgb = get_value("defaults".to_string(), "night_hour_orgb_profile".to_string());
                let ss_srgb = get_value("defaults".to_string(), "screensaver_srgb_profile".to_string());
                let ss_orgb = get_value("defaults".to_string(), "screensaver_orgb_profile".to_string());
                let current_profile = gamemon_value("current_profile".to_string());
                let current_priority = get_value(current_profile.clone(), "priority".to_string())
                    .parse::<u64>().unwrap();
                let section_priority = section.priority.parse::<u64>().unwrap();
                let game_on = match get_value(current_profile.clone(), "game_or_win".to_string()).as_str() {
                    "Game" => true,
                    _ => false
                };
                let display = gamemon_value("display".to_string());

                let path = Path::new("Software").join("GameMon");

                if get_value("Idle".to_string(), "exe_name".to_string()) != ss_get("ScreenSaveTimeOut") {
                    write_key(&"Idle".to_string(), "exe_name", &ss_get("ScreenSaveTimeOut"));
                }

                if user_idle(&wait_time.parse::<u64>().unwrap() - 5)
                    && current_profile == "Idle".to_string() {
                    return;
                }
                
                if user_idle(&wait_time.parse::<u64>().unwrap() - 5)
                    && game_on == false
                    && get_value("Idle".to_string(), "running_pid".to_string()) == "0" { // PAST IDLE TIME!!!!!!
                    
                    let time_range = get_value("Idle".to_string(), "game_window_name".to_string());
                    
                    if dark_hours(&time_range) {
                        log!("Idle detected during dark hours!");
                        run_other_commands(&get_value("Idle".to_string(), "other_commands".to_string()));
                        change_open_rgb(&night_orgb);
                        change_signal_rgb(&night_srgb);
                        change_voice_attack(&get_value("Idle".to_string(), "voice_attack_profile".to_string()));
                        run_ahk(&"Idle".to_string());
                        power_monitors(false);
                        write_key(&"General".to_string(), "running_pid", "0");
                        let _v = reg_write_value(&path
                            , "current_profile".to_string()
                            , "Idle".to_string());
                        write_key(&"Idle".to_string(), "running_pid", "1");

                    } else if display == "off" && dark_hours(&time_range) == false {
                        power_monitors(true);
                        
                    } else if get_value("Idle".to_string(), "game_or_win".to_string()) == "Yes" {

                        log!("Idle detected during daylight hours!");
                        run_other_commands(&get_value("Idle".to_string(), "other_commands".to_string()));
                        change_open_rgb(&ss_orgb);
                        change_signal_rgb(&ss_srgb);
                        change_voice_attack(&get_value("Idle".to_string(), "voice_attack_profile".to_string()));
                        run_ahk(&"Idle".to_string());
                        run_screensaver();
                        write_key(&"General".to_string(), "running_pid", "0");
                        let _v = reg_write_value(&path
                            , "current_profile".to_string()
                            , "Idle".to_string());
                        write_key(&"Idle".to_string(), "running_pid", "1");

                    } else {
                        log!("Idle detected during daylight hours!");
                        run_other_commands(&get_value("Idle".to_string(), "other_commands".to_string()));
                        change_open_rgb(&get_value("Idle".to_string(), "open_rgbprofile".to_string()));
                        change_signal_rgb(&get_value("Idle".to_string(), "signal_rgbprofile".to_string()));
                        change_voice_attack(&get_value("Idle".to_string(), "voice_attack_profile".to_string()));
                        run_ahk(&"Idle".to_string());
                        write_key(&"General".to_string(), "running_pid", "0");
                        let _v = reg_write_value(&path
                            , "current_profile".to_string()
                            , "Idle".to_string());
                        write_key(&"Idle".to_string(), "running_pid", "1");
                    };
                    return;
                } else if user_idle(&wait_time.parse::<u64>().unwrap() - 5) == false
                    && current_profile == "Idle".to_string() {
                        
                    log!(format!("Idle no longer detected!"));
                    power_monitors(true);
                    close_ahk(&"Idle".to_string());
                    write_key(&"Idle".to_string(), "running_pid", "0");
                    let _v = reg_write_value(&path
                        , "current_profile".to_string()
                        , "General".to_string());
                    return;
                }

                if current_priority.cmp(&section_priority) == Ordering::Greater {
                    return;  // DO NOTHING...section is lower priority than current priority
                };
                        
                match section.game_or_win.as_str() {
                    "Game" => {
                        
                        if current_profile == sec && process_exists(Some(&section.exe_name)){
                            return;
                        } else if current_profile != sec && process_exists(Some(&section.exe_name)) {
                            log!(format!("{sec} detected!"));
                            run_other_commands(&section.other_commands);
                            change_open_rgb(&section.open_rgbprofile);
                            change_signal_rgb(&section.signal_rgbprofile);
                            change_voice_attack(&section.voice_attack_profile);
                            run_ahk(&sec);
                            let _v = reg_write_value(&path
                                , "current_profile".to_string()
                                , sec.clone());
                            write_key(&"General".to_string(), "running_pid", "0");
                            close_ahk(&"General".to_string());
                        } else if current_profile == sec && process_exists(Some(&section.exe_name)) == false {
                            log!(format!("{sec} no longer detected!"));
                            close_ahk(&sec);
                            let _v = reg_write_value(&path
                                , "current_profile".to_string()
                                , "General".to_string());
                        }
                    },
                    "Window" => {
                        if current_profile == sec && window_is_active(Some(&section.exe_name)){
                            return;
                        } else if current_profile != sec && window_is_active(Some(&section.exe_name)) {
                            log!(format!("{sec} detected!"));
                            let _v = reg_write_value(&path
                                , "current_profile".to_string()
                                , sec.clone());
                            run_other_commands(&section.other_commands);
                            change_open_rgb(&section.open_rgbprofile);
                            change_signal_rgb(&section.signal_rgbprofile);
                            change_voice_attack(&section.voice_attack_profile);
                            run_ahk(&sec);
                            write_key(&"General".to_string(), "running_pid", "0");
                            close_ahk(&"General".to_string());

                        } else if current_profile == sec && window_is_active(Some(&section.exe_name)) == false {
                            log!(format!("{sec} no longer detected!"));
                            close_ahk(&sec);
                            let _v = reg_write_value(&path
                                , "current_profile".to_string()
                                , "General".to_string());
                        }
                    },
                    _ => (),
                };
            });
            sleep(250);
            thread.join().unwrap();
            
        }; // End Section "For" loop
        
        if gamemon_value("current_profile".to_string()) == "General"
        && get_value("General".to_string(), "running_pid".to_string()) == "0" {
            run_other_commands(&get_value("General".to_string(), "other_commands".to_string()));
            change_open_rgb(&get_value("General".to_string(), "open_rgbprofile".to_string()));
            change_signal_rgb(&get_value("General".to_string(), "signal_rgbprofile".to_string()));
            change_voice_attack(&get_value("General".to_string(), "voice_attack_profile".to_string()));
            run_ahk(&"General".to_string());
            write_key(&"General".to_string(), "running_pid", "1");
        }


    } // End Loop
    
}
                
                

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
