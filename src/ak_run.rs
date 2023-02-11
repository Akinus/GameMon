// Rust Programming Language
// #####################################################################
// File: ak_run.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 13:10:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 11 Feb 2023 @ 15:06:32                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################
//   Import Data ####
use std::{path::Path, process::Command, os::windows::process::CommandExt, cmp::Ordering, sync::{Arc, Mutex}};

use crossbeam::{scope};
use mouse_rs::Mouse;
use sysinfo::{ProcessExt, System, SystemExt};
use ureq::Error;
extern crate winapi;
use winapi::{um::{winbase::CREATE_NO_WINDOW, winuser::{GetForegroundWindow, SetForegroundWindow}}};

use windows_win::{raw::window::{send_message, get_by_class}};
use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE}};

use crate::{ak_utils::{url_encode, macros::{log}, sleep}, ak_io::{read::{get_value, ss_get, get_pid, gamemon_value, Instance}, write::{reg_write_value, write_key}}};

pub fn activate<T>(instruction: (T, Instance)) where T: ToString {
    
    let sec = instruction.0.to_string();
    let section = instruction.1;
    if sec == "" {
        return;
    }

    let binding1 = section.exe_name.clone();
    let binding2 = sec.clone();
    let title = match Some(&binding1){
        Some(o) => o,
        None => &binding2,
    };

    let log_text = match sec.as_str() {
        "General" => Arc::new(
            Mutex::new(
                String::from(format!("Nothing detected! Changing profile to General...\n")))
        ),
        "Idle" => Arc::new(
            Mutex::new(
                String::from(format!("Idle detected! Changing profile to Idle...\n")))
        ),
        _ => Arc::new(
            Mutex::new(
                String::from(format!("{} detected! Changing profile to {}...\n", title, &sec)))
        )
    };

    let _ = scope(|a| {

        let handles = vec![
            {
                let lt = log_text.clone();
                let sec = sec.clone();
                let section = section.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    if sec == "Idle" {
                        if section.game_window_name == "Night" {
                            log_text.push_str(&format!("Idle detected during night hours.\n"));
                            if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                                , "display"
                            ) == "on" {
                                log_text.push_str(&format!("Turning monitors off."));
                                log_text.push_str(&power_monitors(false));
                                log_text.push_str("\n");
                            }
                        } else {
                            log_text.push_str(&format!("Idle detected during day hours.\n"));
                            if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                                , "display"
                            ) == "off" {
                                log_text.push_str(&format!("Turning monitors on."));
                                log_text.push_str(&power_monitors(true));
                                log_text.push_str("\n");
                            }

                            if section.game_or_win == "Yes" {
                                sleep(5000);
                                log_text.push_str(&format!("Activating Screensaver...\n"));
                                run_screensaver();
                            }
                            
                        }
                    }
                })
            },
            {
                let oc = (sec.clone(), section.other_commands.as_str().clone());
                let lt = log_text.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    // Run all other commands
                    log_text.push_str(&run_other_commands(oc));
                    log_text.push_str("\n");
                })
            },
            {
                let vap = section.voice_attack_profile.clone();
                let lt = log_text.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    log_text.push_str(&change_voice_attack(&vap));
                    log_text.push_str("\n");
                })
            },
            {
                
                let ahk = section.path_toahk.clone();
                let lt = log_text.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    log_text.push_str(&run_ahk(&ahk));
                    log_text.push_str("\n");
                })
            },
            {

                let op = section.open_rgbprofile.clone();
                let lt = log_text.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    log_text.push_str(&change_open_rgb(&op));
                    log_text.push_str("\n");
                })
            },
            {
                
                let sp = section.signal_rgbprofile.clone();
                let lt = log_text.clone();
                a.spawn(move |_| {
                    let mut log_text = lt.lock().unwrap();
                    log_text.push_str(&change_signal_rgb(&sp));
                    log_text.push_str("\n");
                })
            },
            {
                
                let sec_clone = sec.clone();
                let cp = section.priority.clone();
                a.spawn(move |_| {
                    // Change current profile to last profile
                let _g = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                    , &Path::new("Software").join("GameMon")
                    , "last_profile".to_string()
                    , gamemon_value(
                            &RegKey::predef(HKEY_LOCAL_MACHINE)
                            , "current_profile".to_string())
                );

                let _p = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                    , &Path::new("Software").join("GameMon")
                    , "current_priority".to_string()
                    , cp
                );
    
                // change current profile
                let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                    , &Path::new("Software").join("GameMon")
                    , "current_profile".to_string()
                    , sec_clone.clone()
                );
    
                // change current profile activated
                let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
                    , &Path::new("Software").join("GameMon")
                    , "current_profile_activated".to_string()
                    , sec_clone.clone()
                );
                })
            }
        ];
        
        for h in handles {
            h.join().unwrap();
        }

        // Log
        log!(format!("{}", log_text.lock().unwrap()));
    });
    
    
}

pub fn deactivate<T>(instruction: (T, Instance)) where T: ToString {
    let sec = instruction.0.to_string();
    let section = instruction.1;
    let mut log_text = format!("{} no longer detected!\n", section.exe_name);
    
    if sec == "" {
        return;
    }

    if sec == "Idle" {
        if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
            , "display"
        ) == "off" {
            log_text.push_str(&format!("\nTurning monitors on."));
            power_monitors(true);
        }
    }
    
    // Close AHK that was running for this section
    log_text.push_str(&close_ahk(&section.path_toahk));

    // Change current profile to last profile
    let _g = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
        , &Path::new("Software").join("GameMon")
        , "last_profile".to_string()
        , sec.clone()
    );

    //change current profile
    let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE)
        , &Path::new("Software").join("GameMon")
        , "current_profile".to_string()
        , "General".to_string()
    );

    //Log
    log!(&format!("{}", log_text));
    
}

// pub fn main_check(enum_keys: &Vec<(String, Instance)>){
//     let (c, p, g) = (
//         gamemon_value(HKEY, "current_profile").to_owned()
//         , gamemon_value(HKEY, "current_profile").to_owned()
//         , gamemon_value(HKEY, "current_profile").to_owned()
//     );

//     if user_idle(get_value(HKEY, "Idle", "exe_name")) {
//         if g != "Idle"{
//             deactivate((g, get_section(p)));
//             activate(("Idle", get_idle()));
//         }
//         return;
//     }

//     if enum_keys.len() > 0 {
//         for entry in enum_keys {
//             let t = entry.clone();
//             if t.0 != g {
//                 deactivate((c.clone(), get_section(p.clone())));
//                 activate(t);
//             }
//             // let _= msg_box(None, Some(&format!("{}", entry.0)), 250);
//         }
//     } else {
//         if g != "General".to_string() {
//             deactivate((c, get_section(p)));
//             activate(("General", get_section("General")));
//         }
//     }
// }

pub fn close_all_ahk() -> Result<(), String> {
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon");
    let game_mon = hklm.open_subkey(&path).unwrap();
    
    for sec in game_mon.enum_keys().map(|x| x.unwrap()){

        match &sec.as_str() {
            &"defaults" => (),
            _ => {
                let ahk_pid = get_ahk_pid(&sec);
                if let Ok(o) = ahk_pid {
                    let close = close_pid(o);
                    assert!(close.is_ok());
                }
                
            }
        }
        
    }
    Ok(())
}

pub fn close_pid(pid: u32) -> Result<std::process::Child, std::io::Error>{
    let kill_cmd = format!("TASKKILL /PID {}", &pid);
    return Command::new("cmd.exe")
    .creation_flags(CREATE_NO_WINDOW)
    .arg("/c")
    .arg(&kill_cmd)
    .spawn();
}

pub fn run_cmd(cmd: &String) -> Result<std::process::Child, std::io::Error>{

    return Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .raw_arg(cmd)
        .spawn();
}

pub fn run_ahk(ahk: &String) -> String {
    let mut return_string = String::new();
    
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .raw_arg(&ahk)
        .spawn();

    if output.is_ok() {
        return_string.push_str(&format!("{ahk} is now running!"));
    } else {
        return_string.push_str(&format!("Error! Could not start {ahk}\n\n{:?}", output.as_ref().unwrap()));
    };

    return_string
}

pub fn close_ahk(ahk: &String) -> String {
    let mut return_string = String::new();
    let mut system = System::new();
    system.refresh_processes();
    let v_ahk_pid = system.processes_by_exact_name("AutoHotkey.exe");
    for p in v_ahk_pid{
        let t_p = p.cmd().to_owned();
        let cmd_line = t_p.last().unwrap();
        if cmd_line.contains(ahk) {
            let pid = p.pid().to_string();
            let output = Command::new("TASKKILL")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/PID")
                .arg(&pid)
                .spawn();
            
                return_string.push_str(&match output {
                    Ok(mut o) => {
                        let status = o.wait().unwrap().code().unwrap();
                        match status{
                            0 => format!("___ Running Command: TASKKILL /PID {} ___\n
SUCCESS!
stdin: {:?}
stdout: {:?}
stderr: {:?}"
                                    , &pid
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr),
                            _ => format!("___ Running Command: TASKKILL /PID {} ___\n
ERROR!
stdin: {:?}
stdout: {:?}
stderr: {:?}"
                                    , &pid
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr),
                        }
                    },
                    Err(e) => format!("Could not kill {}: {e}", &pid)
                });
        }
    };
    return_string
}

pub fn get_ahk_pid(sec: &String) -> Result<u32, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(sec);
    let game_mon = hklm.open_subkey(&path).unwrap();
    let ahk: String = game_mon.get_value("path_toahk").unwrap();

    let binding = System::new_all();
    let v_ahk_pid = binding.processes_by_exact_name("AutoHotkey.exe");
    for p in v_ahk_pid{
        let t_p = p.cmd().to_owned();
        let cmd_line = t_p.last().unwrap();
        if cmd_line.contains(&ahk) {
            return Ok(p.pid().to_string().parse::<u32>().unwrap());
        }
    };
    
    return Err("NONE".to_string())
    
}

pub fn change_signal_rgb(profile: &String) -> String{
    let profile = profile.clone();
    let mut return_string = String::new();

    if profile.is_empty() {
        return_string.push_str("No SignalRGB Profile found for {profile}");
        return return_string;
    }

    let sp = &profile;
    let mut rgb_profile = url_encode(sp.to_string());

    if rgb_profile.contains('?'){
        rgb_profile.push_str("^&-silentlaunch-");
    } else {
        rgb_profile.push_str("?-silentlaunch-");
    }
    
    let command_var = format!("start signalrgb://effect/apply/{}", &rgb_profile);

    let output = run_cmd(&command_var);

    if output.is_ok() {
        return_string.push_str(&format!("Changed SignalRGB to {sp}"));
    } else {
        return_string.push_str(&format!("Could not execute SignalRGB Command: {}: {:?}", &command_var, &output.as_ref().unwrap()));
    }

    return_string
    
}

pub fn change_open_rgb(profile: &String) -> String {
    let profile = profile.clone();
    let mut return_string = String::new();
    
    if profile.is_empty() {
        return_string.push_str("No OpenRGB Profile found for {profile}");
        return return_string;
    }

    let addy = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "orgb_address".to_string());
    let port = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "orgb_port".to_string());
    let rgb_profile = url_encode(profile.to_string());
    let command_var = format!("http://{}:{}/{}", addy, port, &rgb_profile);
    
    return_string.push_str(&match ureq::post(&command_var)
        .set("User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36")
        .set("Content-Type", "application/json")
        .send_string(&format!("Requesting Change OpenRGB profile to {}", &rgb_profile)) {
            Ok(o) => format!("Changed OpenRGB to {}\n\nResponse:\nCode: {}\nContent: {}\n Url: {}",
                &rgb_profile, o.status(), o.status_text(), o.get_url()),
            Err(Error::Status(code, response)) => format!("ERROR: {}", Error::Status(code, response)),
            transport => format!("ERROR: {}", Error::from(transport.unwrap()))
        }
    );

    return_string

}

pub fn change_voice_attack(profile: &String) -> String{
    let profile = profile.clone();
    let mut return_string = String::new();
    
    if profile.is_empty() {
        return_string.push_str("No VoiceAttack profile found.");
        return return_string;
    };
    
    let vac = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "voice_attack_path".to_string());
    let pro = (&profile).to_string();
    let cmd = format!("{} -profile {}", &vac, &pro);

    let output = Command::new(&vac)
        .creation_flags(CREATE_NO_WINDOW)
        .arg("-profile")
        .arg(&pro)
        .spawn();

    if output.is_ok() {
        return_string.push_str(&format!("Changed VoiceAttack profile to {}\n\n{}", &profile, &cmd));
    } else {
        return_string.push_str(&format!("Could not change VoiceAttack profile to {}\n\n{}\nERROR:\n{:?}"
                    , &profile
                    , &cmd
                    , &output.as_ref().unwrap()));
    };

    return_string

}

pub fn power_monitors(on_off: bool) -> String {
    let mut return_string = String::new();
    if on_off {
        //Turn on display
        let mouse = Mouse::new();
        mouse.move_to(0, 0).unwrap();
        mouse.scroll(5).unwrap();
        reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon"),
            "display".to_string(), "on".to_string()).unwrap();
        return_string.push_str("..OK");
    } else {
        //Turn off display
        return_string.push_str(&match send_message(
            get_by_class("Progman",
            None).unwrap()[0],
            0x112,
            0xF170,
            2,
            Some(5)) {
                Ok(_) => {
                    reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon"),
                        "display".to_string(), "off".to_string()).unwrap();
                    String::from("..OK")
                },
                Err(e) => format!("Failed to turn off monitor(s)!! || Error: {}", &e)
        });
    }
    return_string
}

pub fn run_screensaver(){
    sleep(5000);
    let ss_exe = ss_get("SCRNSAVE.EXE").to_owned();
    match get_pid(Some(&ss_exe)) { // Check for Screensaver
        Ok(_) => {
            
            let kill = Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/c")
                .arg("TASKKILL")
                .arg("/im")
                .arg("/f")
                .arg(ss_get("SCRNSAVE.EXE"))
                .spawn();
            match kill {
                Ok(mut o) => {
                    let status = o.wait().unwrap().code().unwrap();
                    match status{
                        0 => {
                            log!(format!("Screensaver Detected...taking ownership of screensaver

SUCCESS!
stdin: TASKKILL /im /f {ss_exe:?}
stdout: {:?}
stderr: {:?}\n"
                                    , o.stdout
                                    , o.stderr)
                            );
                            let output = Command::new("cmd.exe")
                                .creation_flags(CREATE_NO_WINDOW)
                                .arg("/c")
                                .arg(ss_get("SCRNSAVE.EXE"))
                                .arg("/S")
                                .spawn();
                            match output {
                                Ok(mut o) => {
                                    let status = o.wait().unwrap().code().unwrap();
                                    match status{
                                        0 => log!(format!("Initiating Screensaver...\n\n
SUCCESS!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                                        , o.stdout
                                                        , o.stderr)
                                                ),
                                        _ => log!(format!("Initiating Screensaver...\n\n
ERROR!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                                        , o.stdout
                                                        , o.stderr)
                                                    , "e"
                                                ),
                                    }
                                    
                                },
                                Err(e) => {
                                    log!(format!("Could not run {ss_exe:?} /S: {e}\n"), "e")
                                }
                            }
                        },
                _ => log!(format!("___Running Command___

ERROR!
stdin: TASKKILL /im /f {ss_exe}
stdout: {:?}
stderr: {:?}\n"
                                , o.stdout
                                , o.stderr)
                            , "e"
                        ),
                    };
                    
                },
                Err(e) => {
                    log!(format!("Could not run TASKKILL /im /f {ss_exe}: {e}\n"), "e");
                }
            };

        },
        _ => {
            let output = Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/c")
                .arg(ss_get("SCRNSAVE.EXE"))
                .arg("/S")
                .spawn();
            match output {
                Ok(mut o) => {
                    let status = o.wait().unwrap().code().unwrap();
                    match status{
                        0 => log!(format!("Initiating Screensaver...\n\n
SUCCESS!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                        , o.stdout
                                        , o.stderr)
                                ),
                        _ => log!(format!("Initiating Screensaver...\n\n
ERROR!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                        , o.stdout
                                        , o.stderr)
                                    , "e"
                                ),
                    }
            
                },
                Err(e) => {
                    log!(format!("Could not run {ss_exe:?} /S: {e}\n"), "e")
                }
            };
        }
    };
        

        let take_two = std::thread::spawn(move ||{
            sleep(8000);
            match get_pid(Some(&ss_exe)) { // Check for Screensaver
                Ok(_) => return,
                _ => {
                    let output = Command::new("cmd.exe")
                        .creation_flags(CREATE_NO_WINDOW)
                        .arg("/c")
                        .arg(ss_get("SCRNSAVE.EXE"))
                        .arg("/S")
                        .spawn();
                    match output {
                        Ok(mut o) => {
                            let status = o.wait().unwrap().code().unwrap();
                            match status{
                                0 => log!(format!("Initiating Screensaver...\n\n
SUCCESS!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                                , o.stdout
                                                , o.stderr)
                                        ),
                                _ => log!(format!("Initiating Screensaver...\n\n
ERROR!
stdin: {ss_exe:?} /S
stdout: {:?}
stderr: {:?}\n"
                                                , o.stdout
                                                , o.stderr)
                                            , "e"
                                        ),
                            }
                    
                        },
                        Err(e) => {
                            log!(format!("Could not run {ss_exe:?} /S: {e}\n"), "e")
                        }
                    };
                }
            };
        });

        take_two.join().unwrap();

}

pub fn run_other_commands(section_and_commands: (String, &str)) -> String {
    let section = section_and_commands.0;
    let other_commands = section_and_commands.1;
    
    if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "last_other_commands".to_owned()) == section {
        return "".to_string();
    };
    
    if other_commands.is_empty() {
        return "".to_string();
    }

    let mut collection = Vec::new();
    for v in other_commands.split(" && ") {
        collection.push(v.to_owned());
    }

    let wait_time = match (collection.len() as u64).cmp(&1) {
        Ordering::Greater => (collection.len() * 100) as u64,
        _ => (collection.len() * 200) as u64
    };

    let new_section = section.clone().to_owned();
    let window = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), section.to_string(), "game_or_win".to_string());

    
    if section.to_string() != "General" 
        && section != "Idle"
        && window != "Game" 
    {
        std::thread::spawn(move || {
            let h = unsafe { GetForegroundWindow() };
    
            for _ in 0..wait_time {
                let z = unsafe { GetForegroundWindow() };
                if z != h {
                    let _w = unsafe { SetForegroundWindow(h) };
                }
                sleep(10);
            }
        });
    }

    write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");

    let _oc = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
        , "last_other_commands".to_string()
        , new_section.to_string());
    
    let mut final_string = String::new().to_owned();
    
    for c in collection {

        
        write_key(&RegKey::predef(HKEY_LOCAL_MACHINE)
            , &"General".to_string()
            , "running_pid"
            , "1");

        let output = Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .raw_arg(&c)
            .spawn();
        final_string.push_str(&match output {
            Ok(mut o) => {
                let status = o.wait().unwrap().code().unwrap();
                match status{
                    0 => format!("___Running Command___
{}
SUCCESS!
stdin: {:?}
stdout: {:?}
stderr: {:?}\n"
                                    , &c
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr),
                    _ => format!("___Running Command___
{}
ERROR!
stdin: {:?}
stdout: {:?}
stderr: {:?}\n"
                                    , &c
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr)
                }
            },
            Err(e) => format!("Could not run {}: {e}\n", &c)
        });
        
        sleep(250);

    }

    final_string
}
