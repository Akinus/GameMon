// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 10 Dec 2022 @ 22:46:50                          #
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
use sysinfo::{ProcessExt, System, SystemExt, Pid};
use active_win_pos_rs::get_active_window;
use std::{path::{Path}, cmp::Ordering};
use {std::sync::mpsc, tray_item::TrayItem};
use winreg::{enums::*};
use winreg::RegKey;
use windows_win::{raw::window::{
    send_message,
    get_by_class
}};
use winapi::{
    um::{
        winuser::{
            LASTINPUTINFO,
            PLASTINPUTINFO,
            GetLastInputInfo
        },
    }};
use winsafe::{prelude::*};
use winsafe::{HWND, co::{MB}};
use mouse_rs::{Mouse};

mod ak_io;
mod ak_utils;
mod ak_gui;
mod ak_run;
use ak_run::{
    close_all_ahk,
    run_cmd,
    close_pid,
    get_ahk_pid,
    change_open_rgb,
    change_signal_rgb,
    change_voice_attack
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
            get_pid,
            name_by_pid
        },
    logging::initialize_log
};
use ak_utils::{
    sleep,
    Cleanup,
    macros::
        {
            d_quote,
            exit_app,
            log
        },
    Message,
    dark_hours
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
    let _cleanup = Cleanup;

    let last_error = std::io::Error::last_os_error().to_string();
    
    if last_error.contains("GameMon"){
        log!(format!("Last shutdown reason: CRASH"), "e");
    } else {
        log!(format!("Last shutdown reason: {}", get_value("defaults".to_string(), "exit_reason".to_string()).to_string()), "w");
    }

    // Create system tray
    let (tx, rx) = mpsc::channel();

    let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

    tray.add_label("GameMon").unwrap();
    
    tray.add_menu_item("About", || {
        let hwnd = HWND::GetDesktopWindow();
        hwnd.MessageBox(&format!("GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language").to_string()
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

    
    loop {

        
        let mem;
        let hklm;
        let path;
        

        mem = System::new_all().processes_by_exact_name("GameMon.exe").last().unwrap().memory();

        match mem.cmp(&"1073741824".parse::<u64>().unwrap()){
            Ordering::Greater => {
                exit_app!(0, "Memory allocation too high");
            },
            _ => ()
        }

        'channel: loop {
            match rx.try_recv(){
                Ok(Message::Quit) => exit_app!(1, "Menu"),
                Ok(Message::Gui) => {
                    std::thread::spawn(|| {
                        main_gui();
                    });
                },
                Ok(Message::Defaults) => {
                    std::thread::spawn(||{
                        defaults_gui();
                    });
                },
                Ok(Message::Logs) => {
                    let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
                },
                Err(_) => break 'channel,
            };
        }

        hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        path = Path::new("Software").join("GameMon");
        
        for sec in hklm.open_subkey(path).unwrap().enum_keys().map(|x| x.unwrap()){

            std::thread::spawn(move || {
            
                let section;
                // let defaults = get_defaults();
                let time_range;
                let ss;
                let cmds;
                let ahk_run;
                let ss_exe;
                let ahk_pid;
                let ahk_pid_u32;
                let ahk_close;
                let game_bool;
                let win_flag;
                let active_pid;
                let active_win;
                let current_priority = get_value("defaults".to_string(), "current_priority".to_string());
                let game_on = get_value("defaults".to_string(), "gameon".to_string());
                let night_srgb = get_value("defaults".to_string(), "night_hour_srgb_profile".to_string());
                let night_orgb = get_value("defaults".to_string(), "night_hour_orgb_profile".to_string());
                let ss_srgb = get_value("defaults".to_string(), "screensaver_srgb_profile".to_string());
                let ss_orgb = get_value("defaults".to_string(), "screensaver_orgb_profile".to_string());
                let window_flag = get_value("defaults".to_string(), "window_flag".to_string());


                let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                let path = Path::new("Software").join("GameMon");
                let game_mon = hklm.open_subkey(path).unwrap();

                section = match &sec.as_str() {
                    &"defaults" => return,
                    &"General" => return,
                    _ => get_section(&sec),
                };
                
                match &current_priority.parse::<u64>().unwrap().cmp(&section.priority.parse::<u64>().unwrap()) {
                    Ordering::Greater => return, // DO NOTHING...section is lower priority than current priority
                    _ => ()
                };
                
                match &sec.as_str() {
                    &"Idle" => { // Begin Idle Reaction
                        write_key(&sec, "exe_name", &ss_get("ScreenSaveTimeOut"));
                        let now = unsafe { winapi::um::sysinfoapi::GetTickCount() };
                        let mut last_input_info = LASTINPUTINFO {
                            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                            dwTime: 0
                        };
                    
                        let p_last_input_info: PLASTINPUTINFO = &mut last_input_info as *mut LASTINPUTINFO;
                    
                        let ok = unsafe { GetLastInputInfo(p_last_input_info) } != 0;
                    
                        let idle_seconds = match ok {
                            true => {
                                let millis = now - last_input_info.dwTime;
                                Ok(std::time::Duration::from_millis(millis as u64))
                            },
                            false => Err(format!("GetLastInputInfo failed"))
                        }.unwrap().as_secs();
                        time_range = section.game_window_name;

                        match idle_seconds.cmp(&(&section.exe_name.parse::<u64>().unwrap() - 5)) {
                            Ordering::Greater => { // PAST IDLE TIME!!!!!!
                        
                                match &section.running.as_str() {
                                    &"False" => { //IDLE IS NOT RUNNING
                                        
                                        
                                        match &game_on.as_str() {
                                            &"True" => return,
                                            _ => ()
                                        };

                                        //change values
                                        write_key(&"defaults".to_string(), "current_priority", &section.priority);
                                        write_key(&sec, "running", "True");
                                        write_key(&sec, "running_pid", "0");
                                        write_key(&"General".to_string(), "running", "False");

                                        if dark_hours(&time_range) == true { // WITHIN DARK HOURS!!!!!
                                            log!(&format!("Idle detected! Within dark hours!\nDark Hours are between {}", &time_range));

                                            //change profiles
                                            log!(change_signal_rgb(&night_srgb));
                                            log!(change_open_rgb(&night_orgb).unwrap());

                                            // Run other commands                                        
                                            let _z = match send_message(
                                                get_by_class("Progman",
                                                None).unwrap()[0],
                                                0x112,
                                                0xF170,
                                                2,
                                                Some(5)) {
                                                    Ok(_) => {
                                                        reg_write_value(&Path::new("Software").join("GameMon"),
                                                            "display".to_string(), "off".to_string()).unwrap();
                                                        String::from("OK")
                                                    },
                                                    Err(e) => log!(format!("Failed to turn off monitor(s)!! || Error: {}", &e), "e")
                                            };

                                            match &section.other_commands.as_str() {
                                                &"" => (),
                                                s => {
                                                    for c in s.split(" && ") {
                                                        let _f = match run_cmd(&c.to_string()) {
                                                            Ok(_) => log!(format!("Running {}", &c)),
                                                            Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                        };
                                                    }
                                                }
                                            }

                                        } else { // NOT WITHIN DARK HOURS!!!!
                                            log!(&format!("Idle detected! Within day hours!\nDay hours are hours outside of the range {}", &time_range));

                                            //change values
                                            write_key(&"General".to_string(), "running", "False");
                                            write_key(&"defaults".to_string(),
                                                "current_priority", &section.priority);

                                            //Make sure display is on
                                            log!("Ensuring Display is ON.");
                                            reg_write_value(&Path::new("Software").join("GameMon"),
                                                "display".to_string(), "on".to_string()).unwrap();

                                            // Run Screensaver
                                            ss_exe = ss_get("SCRNSAVE.EXE");
                                            match get_pid(Some(&ss_exe)) { // Check for Screensaver
                                                Ok(_) => {
                                                    match section.game_or_win.as_str() {
                                                        "Yes" => {
                                                            
                                                            let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                            let r = run_cmd(cmd);
                                                            log!(format!("Taking ownership of screensaver...\n\n{:?}", &r));
                                                            
                                                            //change profiles
                                                            log!(change_signal_rgb(&ss_srgb));
                                                            log!(change_open_rgb(&ss_orgb).unwrap());
                                                            
                                                            log!("Running Screensaver...");
                                                            ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                            let _z = match run_cmd(&ss) {
                                                                Ok(_) => String::from("OK"),
                                                                Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                            };
                                                        },
                                                        _ => {
                                                            let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                            let _r = run_cmd(cmd);
                                                            //change profiles
                                                            match section.running_pid.as_str() {
                                                                "0" => {
                                                                    log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                    log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                                    write_key(&sec, "running_pid", "1");
                                                                },
                                                                _ => (),
                                                            }
                                                            
                                                        },
                                                    }
                                                    
                                                },
                                                _ => {
                                                    // Run Screensaver
                                                    match section.game_or_win.as_str() {
                                                        "Yes" => {
                                                            log!(&format!("Idle is running but screensaver not detected!"));
                                                            //change profiles
                                                            log!("Running Screensaver...");
                                                            log!(change_signal_rgb(&ss_srgb));
                                                            log!(change_open_rgb(&ss_orgb).unwrap());
                                                            
                                                            ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                            let _z = match run_cmd(&ss) {
                                                                Ok(_) => String::from("OK"),
                                                                Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                            };
                                                        },
                                                        _ => {
                                                            //change profiles
                                                            match section.running_pid.as_str() {
                                                                "0" => {
                                                                    log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                    log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                                    write_key(&sec, "running_pid", "1");
                                                                },
                                                                _ => (),
                                                            }
                                                        },
                                                    }
                                                }
                                            };

                                            // Run other commands
                                            match &section.other_commands.as_str() {
                                                &"" => (),
                                                s => {
                                                    cmds = s.split(" && ");
                                                    for c in cmds {
                                                        let _f = match run_cmd(&c.to_string()) {
                                                            Ok(_) => log!(format!("Running {}", &c)),
                                                            Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                        };
                                                    }
                                                }
                                            }
                                            
                                        };
                                        ahk_run = run_cmd(&section.path_toahk);
                                        assert!(ahk_run.is_ok());
                                        log!(&format!("{} is running!", section.name_ofahk));

                                    },
                                    _ => { // Idle is running!
                                        
                                        if dark_hours(&time_range) == true { // WITHIN DARK HOURS!!!!!                                       
                                            let _p = match d_quote!(game_mon.get_raw_value("display").unwrap().to_string().as_str()).as_str() {
                                                "on" => { // Display is on past dark hours
                                                    //Turn off display
                                                    let _z = match send_message(
                                                        get_by_class("Progman",
                                                        None).unwrap()[0],
                                                        0x112,
                                                        0xF170,
                                                        2,
                                                        Some(5)) {
                                                            Ok(_) => {
                                                                reg_write_value(&Path::new("Software").join("GameMon"),
                                                                    "display".to_string(), "off".to_string()).unwrap();
                                                                String::from("OK")
                                                            },
                                                            Err(e) => log!(format!("Failed to turn off monitor(s)!! || Error: {}", &e), "e")
                                                    };
                                                    //change profiles
                                                    log!(change_signal_rgb(&night_srgb));
                                                    log!(change_open_rgb(&night_orgb).unwrap());
                                                },
                                                _ => ()
                                            };
                                        } else { // Not WITHIN dark hours!    
                                            ss_exe = ss_get("SCRNSAVE.EXE");                                    
                                            let _p = match d_quote!(game_mon.get_raw_value("display").unwrap().to_string().as_str()).as_str() {
                                                "off" => { // Display is off during day hours                    
                                                    log!(&format!("Idle is running; Day Hours Activated.  Turning on Display"));
                                                    //Turn on display
                                                    let mouse = Mouse::new();
                                                    mouse.move_to(0, 0).expect("Failed to turn on monitor(s)!!");
                                                    mouse.scroll(5).expect("Failed to scroll wheel!");
                                                    reg_write_value(&Path::new("Software").join("GameMon"),
                                                        "display".to_string(), "on".to_string()).unwrap();
                                                    
                                                    match section.game_or_win.as_str() {
                                                        "Yes" => {
                                                            //change profiles
                                                            match section.running_pid.as_str() {
                                                                "0" => {
                                                                    log!(change_signal_rgb(&ss_srgb));
                                                                    log!(change_open_rgb(&ss_orgb).unwrap());
                                                                    write_key(&sec, "running_pid", "1");
                                                                },
                                                                _ => (),
                                                            }
                                                            
                                                        },
                                                        _ => {
                                                            let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                            let _r = run_cmd(cmd);
                                                            //change profiles
                                                            match section.running_pid.as_str() {
                                                                "0" => {
                                                                    log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                    log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                                    write_key(&sec, "running_pid", "1");
                                                                },
                                                                _ => (),
                                                            }
                                                            
                                                        },
                                                    }
                                                },
                                                _ => {                            
                                                    match get_pid(Some(&ss_exe)) { // Check for Screensaver
                                                        Ok(_) => {
                                                            match section.game_or_win.as_str() {
                                                                "Yes" => {
                                                                    
                                                                    //change profiles
                                                                    match section.running_pid.as_str() {
                                                                        "0" => {
                                                                            let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                                            let _r = run_cmd(cmd);

                                                                            ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                                            let _z = match run_cmd(&ss) {
                                                                                Ok(_) => String::from("OK"),
                                                                                Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                                            };
                                                                            log!(change_signal_rgb(&ss_srgb));
                                                                            log!(change_open_rgb(&ss_orgb).unwrap());
                                                                            write_key(&sec, "running_pid", "1");
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                },
                                                                _ => {
                                                                    let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                                    let _r = run_cmd(cmd);
                                                                    //change profiles
                                                                    match section.running_pid.as_str() {
                                                                        "0" => {
                                                                            log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                            log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                                            write_key(&sec, "running_pid", "1");
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                    
                                                                },
                                                            }
                                                        },
                                                        _ => {
                                                            // Run Screensaver
                                                            match section.game_or_win.as_str() {
                                                                "Yes" => {
                                                                    log!(&format!("Idle is running but screensaver not detected!"));
                                                                    //change profiles
                                                                    log!(change_signal_rgb(&ss_srgb));
                                                                    log!(change_open_rgb(&ss_orgb).unwrap());
                                                                    
                                                                    log!("Running Screensaver...");
                                                                    ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                                    let _z = match run_cmd(&ss) {
                                                                        Ok(_) => String::from("OK"),
                                                                        Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                                    };
                                                                },
                                                                _ => {
                                                                    //change profiles
                                                                    match section.running_pid.as_str() {
                                                                        "0" => {
                                                                            log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                            log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                                            write_key(&sec, "running_pid", "1");
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                    
                                                                },
                                                            }
                                                        }
                                                    };
                                                }
                                            };
                                            
                                        };
                                    }
                                }
                            },
                            _ => { // NOT PAST IDLE TIME
                                reg_write_value(&Path::new("Software").join("GameMon"),
                                    "display".to_string(), "on".to_string()).unwrap();

                                match &section.running.as_str() {
                                    &"True" => { //IDLE IS RUNNING
                                        log!(&format!("Idle no longer detected!"));

                                        //change values
                                        write_key(&"General".to_string(), "running", "True");
                                        write_key(&sec, "running", "False");
                                        write_key(&sec, "running_pid", "0");
                                        write_key(&"defaults".to_string(), "current_priority", "0");

                                        //run extra commands
                                        ahk_pid = get_ahk_pid(&sec.to_string());
                                        assert!(ahk_pid.is_ok());
                                        ahk_pid_u32 = ahk_pid.unwrap();

                                        ahk_close = close_pid(ahk_pid_u32);
                                        assert!(ahk_close.is_ok());
                                        log!(&format!("{} is no longer running!", section.name_ofahk));
                                        
                                        log!(&format!("{}", reset_running()));
                                    },
                                    _ => ()
                                }
                            }
                        };
                    }, // End Idle Reaction

                    _ => { // Begin ALL OTHER Reaction

                        match section.game_or_win.as_str() {
                            "Game" => {
                                
                                //is program running?
                                
                                game_bool = get_pid(Some(&section.exe_name));
                                match game_bool {
                                    Ok(0) => { //Program not found
                                        match &section.running.as_str() {
                                            &"True" => { //Profile is on
                                                log!(&format!("{} no longer detected!", &section.exe_name));

                                                //change values
                                                write_key(&"General".to_string(), "running", "True");
                                                write_key(&sec, "running", "False");
                                                write_key(&"defaults".to_string(), "gameon", "False");
                                                write_key(&"defaults".to_string(), "current_priority", "0");

                                                //run extra commands
                                                ahk_pid = get_ahk_pid(&sec.to_string());
                                                assert!(ahk_pid.is_ok());
                                                ahk_pid_u32 = ahk_pid.unwrap();

                                                ahk_close = close_pid(ahk_pid_u32);
                                                assert!(ahk_close.is_ok());
                                                log!(&format!("{} is no longer running!", section.name_ofahk));
                                                
                                                log!(&format!("{}", reset_running()));   
                                                
                                            },
                                            _ => (),
                                        }; 
                                    },
                                    Ok(msg) => { //Program Found
                                        match &section.running.as_str() {
                                            &"False" => { // Profile Off
                                                log!(&format!("{} detected!", &section.exe_name));

                                                //change values
                                                write_key(&sec, "running", "True");
                                                write_key(&sec, "running_pid", &msg.to_string());
                                                write_key(&"General".to_string(), "running", "False");
                                                write_key(&"defaults".to_string(), "gameon", "True");
                                                write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                                //change profiles
                                                log!(change_signal_rgb(&section.signal_rgbprofile));
                                                log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                
                                                match &section.voice_attack_profile.as_str() {
                                                    &"" => log!("No VoiceAttack profile found."),
                                                    _ => log!(&change_voice_attack(&section.voice_attack_profile))
                                                };

                                                //run extra commands
                                                ahk_run = run_cmd(&section.path_toahk);
                                                assert!(ahk_run.is_ok());
                                                log!(&format!("{} is running!", &section.name_ofahk));
                                                match &section.other_commands.as_str() {
                                                    &"" => (),
                                                    s => {
                                                        cmds = s.split(" && ");
                                                        for c in cmds {
                                                            let _f = match run_cmd(&c.to_string()) {
                                                                Ok(_) => log!(format!("Running {}", &c)),
                                                                Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                            };
                                                        }
                                                    }
                                                }
                                                
                                                
                                            },
                                            _ => ()
                                        };
                                    },
                                    _ => {
                                        
                                    }
                                };

                            },
                            "Window" => {
                                // is a game running?
                                match &game_on.as_str() {
                                    &"True" => return,
                                    _ => ()
                                };

                                match &get_value("Idle".to_string(), "running".to_owned()).as_str() {
                                    &"True" => return,
                                    _ => ()
                                }

                                win_flag = &window_flag;

                                // is window active?
                                active_pid = get_active_window().unwrap().process_id;
                                active_win = name_by_pid(Pid::from(active_pid as usize)).unwrap();

                                if &active_win == &section.exe_name{ // ** WINDOW IS ACTIVE **
                                    match &section.running.as_str() {
                                        &"False" => { //Profile Off
                                            log!(&format!("{} detected!", &section.exe_name));
                                            
                                            //change values
                                            write_key(&sec, "running", "True");
                                            write_key(&sec, "running_pid", &active_pid.to_string());
                                            write_key(&"General".to_string(), "running", "False");
                                            write_key(&"defaults".to_string(), "window_flag", &sec);
                                            write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                            //change profiles
                                            log!(change_signal_rgb(&section.signal_rgbprofile));
                                            log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                            
                                            match &section.voice_attack_profile.as_str() {
                                                &"" => log!("No VoiceAttack profile found."),
                                                _ => log!(&change_voice_attack(&section.voice_attack_profile))
                                            };

                                            //run extra commands
                                            ahk_run = run_cmd(&section.path_toahk);
                                            assert!(ahk_run.is_ok());
                                            log!(&format!("{} is running!", &section.name_ofahk));
                                            match &section.other_commands.as_str() {
                                                &"" => (),
                                                s => {
                                                    cmds = s.split(" && ");
                                                    for c in cmds {
                                                        let _f = match run_cmd(&c.to_string()) {
                                                            Ok(_) => log!(format!("Running {}", &c)),
                                                            Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                        };
                                                    }
                                                }
                                            }
                                            
                                            
                                        },
                                        _ => {
                                            if &win_flag == &&sec {
                                                return;
                                            } else {
                                                write_key(&"defaults".to_string(), "window_flag", &sec);
                                                write_key(&"General".to_string(), "running", "False");
                                                write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                                //change profiles
                                                log!(change_signal_rgb(&section.signal_rgbprofile));
                                                log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                                                
                                            }
                                        }
                                    };
                                } else { // ** WINDOW IS NOT ACTIVE **
                                    match &section.running.as_str() {
                                        &"True" => { // Profile On
                                            log!(&format!("{} no longer detected!", &section.exe_name));

                                            //change values
                                            write_key(&sec, "running", "False");
                                            if &win_flag == &&sec {
                                                write_key(&"defaults".to_string(), "window_flag", "General");
                                                write_key(&"General".to_string(), "running", "True");
                                                write_key(&"defaults".to_string(), "current_priority", "0");

                                            }

                                            //run extra commands
                                            ahk_pid = get_ahk_pid(&sec.to_string());
                                            assert!(ahk_pid.is_ok());
                                            ahk_pid_u32 = ahk_pid.unwrap();

                                            ahk_close = close_pid(ahk_pid_u32);
                                            assert!(ahk_close.is_ok());
                                            log!(&format!("{} is no longer running!", section.name_ofahk));

                                        },
                                        _ => {
                                            if &win_flag == &&sec {
                                                write_key(&"defaults".to_string(), "window_flag", "General");                                            
                                            }
                                        }
                                    };
                                }
                            }
                            _ => ()
                        };
                    } //End Game or Window Reaction
                    
                }; // End of section match

            }); //End of Thread
            sleep(150);
        }; // End of section "For" loop
        let section = get_section(&"General".to_string());
        if &section.running == "True" {
            if &section.running_pid == "0" {
                //change profiles
                log!(change_signal_rgb(&section.signal_rgbprofile));
                log!(change_open_rgb(&section.open_rgbprofile).unwrap());
                
                match &section.voice_attack_profile.as_str() {
                    &"" => log!("No VoiceAttack profile found."),
                    _ => log!(&change_voice_attack(&section.voice_attack_profile))
                };
            };
                
                write_key(&"General".to_string(), "running_pid", "1");
                write_key(&"defaults".to_string(), "current_priority", "0");
        } else {
            match section.running_pid.as_str() {
                "0" => (),
                _ => {
                    write_key(&"General".to_string(), "running_pid", "0");
                }

            };
        };
    };
    
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
