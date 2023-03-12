// Rust Programming Language
// #####################################################################
// File: ak_run.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 13:10:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Wed, 08 Mar 2023 @ 18:45:11                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################
//   Import Data ####
use std::{
    cmp::Ordering,
    os::windows::process::CommandExt,
    path::{Path},
    process::Command,
    sync::{Arc, Mutex},
};

use sysinfo::{Process, ProcessExt, System, SystemExt};
use ureq::Error;
extern crate winapi;
use winapi::{
    um::{
        winbase::CREATE_NO_WINDOW,
        winuser::{
            GetForegroundWindow,
            SetForegroundWindow,
            WM_CLOSE,
            WM_SYSCOMMAND,
            SC_MONITORPOWER            
        },
    },
};

use windows_win::raw::window::{get_by_class, get_by_pid, send_message};
use winreg::{enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER}, RegKey};

use crate::{
    ak_io::{
        read::{
            gamemon_value, get_pid, get_section, get_value, ss_get, user_idle, Instance,
        },
        write::{reg_write_value, write_key},
    },
    ak_utils::{
        macros::{log},
        sleep, url_encode, HKEY,
    },
};

pub fn activate<T>(instruction: (T, Instance))
where
    T: ToString,
{
    let sec = instruction.0.to_string();
    let section = instruction.1;
    if sec == "" {
        return;
    }

    let binding1 = section.exe_name.clone();
    let binding2 = sec.clone();
    let title = match Some(&binding1) {
        Some(o) => o,
        None => &binding2,
    };

    let current_profile = gamemon_value(HKEY, "current_profile");

    let log_text = match sec.as_str() {
        "General" => Arc::new(Mutex::new(String::from(format!(
            "Nothing detected! Changing profile to General...\n{}\n\n",
                &deactivate(current_profile)
        )))),
        "Idle" => Arc::new(Mutex::new(String::from(format!(
            "Idle detected! Changing profile to Idle...\n{}\n\n",
            &deactivate(current_profile)
        )))),
        _ => Arc::new(Mutex::new(String::from(format!(
            "{} detected! Changing profile to {}...\n{}\n\n",
            title, &sec, &deactivate(current_profile)
        )))),
    };

    
    let handles = vec![
        {
            let oc = (sec.clone(), section.other_commands.clone());
            let section = section.clone();
            let sec = sec.clone();
            let lt = log_text.clone();
            std::thread::spawn(move || {
                let mut log_text = lt.lock().unwrap();
                // Run all other commands
                log_text.push_str(&run_other_commands(oc));
                
                if sec == "Idle" {
                    log_text.push_str("\n_____ Idle Special Settings _____\n");
                    if section.game_window_name == "Night" {
                        log_text.push_str(&format!("Idle detected during night hours.\n"));
                        if gamemon_value(HKEY, "display") == "on" && user_idle()
                        {
                            log_text.push_str(&format!("Turning monitors off.{}\n", &power_monitors(false)));
                        }
                    } else {
                        log_text.push_str(&format!("Idle detected during day hours.\n"));
                        if gamemon_value(HKEY, "display") == "off" && user_idle()
                        {
                            log_text.push_str(&format!("Turning monitors on.{}\n", &power_monitors(true)));
                        }

                        if section.game_or_win == "Yes" && user_idle(){
                            reg_write_value(
                                &RegKey::predef(HKEY_CURRENT_USER),
                                &Path::new("Control Panel").join("Desktop"),
                                "ScreenSaveActive".to_string(),
                                "0".to_string()).unwrap()
                            ;
                            log_text.push_str(&format!("Activating Screensaver...\n"));
                            log_text.push_str(&run_screensaver());
                            log_text.push_str("\n");
                        }
                    }
                    log_text.push_str("\n");
                }
                
            })
        },
        {
            let vap = section.voice_attack_profile.clone();
            let lt = log_text.clone();
            std::thread::spawn(move || {
                let mut log_text = lt.lock().unwrap();
                log_text.push_str("\n_____ Voice Attack _____\n");
                log_text.push_str(&change_voice_attack(&vap));
                log_text.push_str("\n\n");
            })
        },
        {
            let ahk = section.path_toahk.clone();
            let lt = log_text.clone();
            std::thread::spawn(move || {
                let mut log_text = lt.lock().unwrap();
                log_text.push_str("\n_____ Autohotkey Scripts _____\n");
                log_text.push_str(&run_ahk(&ahk));
                log_text.push_str("\n\n");
            })
        },
        {
            let op = section.open_rgbprofile.clone();
            let lt = log_text.clone();
            std::thread::spawn(move || {
                let mut log_text = lt.lock().unwrap();
                log_text.push_str("\n_____ OpenRGB _____\n");
                log_text.push_str(&change_open_rgb(&op));
                log_text.push_str("\n\n");
            })
        },
        {
            let sp = section.signal_rgbprofile.clone();
            let lt = log_text.clone();
            std::thread::spawn(move || {
                let mut log_text = lt.lock().unwrap();
                log_text.push_str("\n_____ SignalRGB _____\n");
                log_text.push_str(&change_signal_rgb(&sp));
                log_text.push_str("\n\n");
            })
        },
        {
            let sec_clone = sec.clone();
            let cp = section.priority.clone();
            std::thread::spawn(move || {

                // Change current priority
                let _p = reg_write_value(
                    &RegKey::predef(HKEY_LOCAL_MACHINE),
                    &Path::new("Software").join("GameMon"),
                    "current_priority".to_string(),
                    cp,
                );

                // change current profile
                let _v = reg_write_value(
                    &RegKey::predef(HKEY_LOCAL_MACHINE),
                    &Path::new("Software").join("GameMon"),
                    "current_profile".to_string(),
                    sec_clone.clone(),
                );

                // change current profile activated
                let _v = reg_write_value(
                    &RegKey::predef(HKEY_LOCAL_MACHINE),
                    &Path::new("Software").join("GameMon"),
                    "current_profile_activated".to_string(),
                    sec_clone.clone(),
                );
            })  
        }
    ];
    
    for h in handles {
        h.join().unwrap();
    }

    // Log
    log!(format!("{}", log_text.lock().unwrap()));
}

pub fn deactivate<T>(instruction: T) -> String
where
    T: ToString,
{
    let sec = instruction.to_string();
    let section = get_section(&sec);
    let mut log_text = String::new();

    if sec == "" {
        return log_text;
    }

    if sec == "General" {
        log_text.push_str(&format!("\nDeactivating the General profile.\n"));
    } else if sec == "Idle" {
        log_text.push_str(&format!("\nDeactivating the Idle profile.\n"));
        if gamemon_value(HKEY, "display") == "off" && !user_idle(){
            log_text.push_str(&format!("\nTurning monitors on."));
            power_monitors(true);
        }
        if section.game_or_win == "Yes" && !user_idle(){
            reg_write_value(
                &RegKey::predef(HKEY_CURRENT_USER),
                &Path::new("Control Panel").join("Desktop"),
                "ScreenSaveActive".to_string(),
                "1".to_string()).unwrap()
        }

        let ss_exe = ss_get("SCRNSAVE.EXE").to_owned();
        if ss_exe.contains("Lively.scr") {
            log_text.push_str(&format!("Lively.scr is designated as the screensaver.  Running \"Livelycu.exe screensaver --show false\"\n"));
            let _ = Command::new("Livelycu.exe")
                .arg("screensaver")
                .arg("--show")
                .arg("false")
                .creation_flags(CREATE_NO_WINDOW)
                .output()
            ;
        }
    } else {
        log_text.push_str(&format!("\nDeactivating {}. {} is no longer detected!\n", &sec, section.exe_name.clone()));
    }

    // Close AHK that was running for this section
    log_text.push_str(&close_ahk(&section.path_toahk));

    // Change current profile to last profile
    let _g = reg_write_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        &Path::new("Software").join("GameMon"),
        "last_profile".to_string(),
        sec.clone(),
    );

    //Log
    return format!("{}", log_text);
}

// pub fn main_check(rx: Receiver<Message>) {
//     // main_check();
//     let (exit_tx, exit_rx) = channel::bounded(2);
//     let mut enum_keys;
//     enum_keys = RegKey::predef(HKEY_LOCAL_MACHINE)
//         .open_subkey(&PathBuf::from("Software").join("GameMon"))
//         .unwrap()
//         .enum_keys()
//         .map(|x| {
//             let y = x.unwrap().clone();
//             let z = y.clone();
//             (z, get_section(y))
//         })
//         .filter(|entry| {
//             entry.0 != "Idle".to_string() && entry.0 != "General" && entry.0 != "defaults"
//         })
//         .collect::<Vec<(String, Instance)>>();
//     let mut current_profile = gamemon_value(HKEY, "current_profile").to_owned();
//     let mut current_priority = gamemon_value(HKEY, "current_priority").to_owned();

//     let mut count = 0;

//     let mut window_handle = get_by_title("rust_systray_window", None).unwrap();

//     if user_idle() {
//         if &current_profile != "Idle" {
//             let idle = get_idle();
//             current_priority = idle.priority.clone();
//             deactivate(&current_profile);
//             activate(("Idle", idle));
//             current_profile = "Idle".to_string();
//         }
//         return;
//     }

//     let f_keys = filtered_keys(enum_keys.clone(), &current_profile, &current_priority);
//     if f_keys.len() > 0 {
//         for (sec, section) in &f_keys {
//             if sec != &current_profile {
//                 deactivate(&current_profile);
//                 current_priority = section.priority.clone();
//                 activate((&sec, section.clone()));
//                 current_profile = sec.clone();
//             }
//         }
//     } else {
//         if &current_profile != "General" {
//             deactivate(&current_profile);
//             let general = get_section("General");
//             current_priority = general.priority.clone();
//             activate(("General", general));
//             current_profile = "General".to_string();
//         }
//     }

//     for h in &window_handle {
//         if !h.is_null() {
//             let _ = msg_box("", format!("{:?}", window_handle), 1500);

//             // let _ = msg_box("", format!("{:?}\n{}", h, msg.unwrap().message), 1500);
//             // match msg.unwrap().message {
//             //     WM_CLOSE => exit_tx.send(WM_CLOSE).unwrap(),
//             //     WM_QUIT => exit_tx.send(WM_QUIT).unwrap(),
//             //     WM_DESTROY => exit_tx.send(WM_DESTROY).unwrap(),
//             //     WM_ENDSESSION => exit_tx.send(WM_ENDSESSION).unwrap(),
//             //     _ => ()
//             // }
//         } else {
//             let _ = msg_box("", "NULL!!!!!", 1500);
//         };
//     }

//     match rx.try_recv() {
//         Ok(Message::Quit) => {
//             exit_tx.send(1).unwrap();
//         }
//         Ok(Message::Gui) => {
//             scope(|s| {
//                 s.spawn(|_| main_gui());
//             })
//             .unwrap();
//         }
//         Ok(Message::Defaults) => {
//             scope(|s| {
//                 s.spawn(|_| defaults_gui());
//             })
//             .unwrap();
//         }
//         Ok(Message::Logs) => {
//             let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
//         }
//         Err(_) => (),
//     };

//     match exit_rx.try_recv() {
//         Ok(0) => {
//             exit_app!(0, "Memory allocation too high!!");
//         }
//         Ok(1) => {
//             exit_app!(1, "Menu");
//         }
//         Ok(WM_CLOSE) => {
//             exit_app!(1, "Window Messsage: WM_CLOSE");
//         }
//         Ok(WM_QUIT) => {
//             exit_app!(1, "Window Messsage: WM_QUIT");
//         }
//         Ok(WM_DESTROY) => {
//             exit_app!(1, "Window Messsage: WM_DESTROY");
//         }
//         Ok(WM_ENDSESSION) => {
//             exit_app!(1, "Window Messsage: WM_ENDSESSION");
//         }
//         Ok(_) => (),
//         Err(_) => (),
//     }
// }

pub fn close_all_ahk() -> Result<(), String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon");
    let game_mon = hklm.open_subkey(&path).unwrap();

    for sec in game_mon.enum_keys().map(|x| x.unwrap()) {
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

pub fn close_pid(pid: u32) -> Result<std::process::Child, std::io::Error> {
    let kill_cmd = format!("TASKKILL /PID {}", &pid);
    return Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg(&kill_cmd)
        .spawn();
}

pub fn run_cmd<T>(cmd: T) -> Result<String, String>
where
    T: ToString,
{
    let cmd = cmd.to_string();
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .raw_arg(&cmd)
        .spawn();
    let return_string = match output {
        Ok(mut o) => {
            let status = o.wait().unwrap().code().unwrap();
            match status{
                0 => Ok(format!("\n--------------------\n{}\nSUCCESS!\nstdin: {:?}\nstdout: {:?}\nstderr: {:?}\n"
                            , &cmd
                            , o.stdin
                            , o.stdout
                            , o.stderr
                        )),
                _ => Err(format!("\n--------------------\n{}\nERROR!\nstdin: {:?}\nstdout: {:?}\nstderr: {:?}\n"
                            , &cmd
                            , o.stdin
                            , o.stdout
                            , o.stderr
                        )),
            }
        }
        Err(e) => Err(format!("Could not run {}: {e}\n", &cmd)),
    };

    return_string
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
        return_string.push_str(&format!(
            "Error! Could not start {ahk}\n\n{:?}",
            output.as_ref().unwrap()
        ));
    };

    return_string
}

pub fn close_ahk(ahk: &String) -> String {
    let mut return_string = String::new();
    let mut s = System::new();
    s.refresh_processes();

    let processes = s.processes_by_name("AutoHotkey").collect::<Vec<&Process>>();

    if processes.is_empty() {
        return_string.push_str(&format!("No window matching {} found!\n", &ahk));
        return return_string;
    }

    for p in processes {
        if p.cmd().contains(&ahk) {
            let handle = get_by_pid(p.pid().to_string().parse::<u32>().unwrap()).unwrap();

            if handle.is_none() {
                return_string.push_str(&format!("No window matching {} found!\n", &ahk));
                return return_string;
            } else {
                return_string.push_str(&format!("\nClosing {}...", &ahk));
            }

            let result = send_message(handle.unwrap(), WM_CLOSE, 0, 0, None);

            let r = match result {
                Ok(_) => "OK!".to_string(),
                Err(_) => format!("ERROR!! Could not close {}\n", &ahk),
            };

            return_string.push_str(&r);
        }
    }

    return_string
}

pub fn get_ahk_pid(sec: &String) -> Result<u32, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(sec);
    let game_mon = hklm.open_subkey(&path).unwrap();
    let ahk: String = game_mon.get_value("path_toahk").unwrap();

    let binding = System::new_all();
    let v_ahk_pid = binding.processes_by_exact_name("AutoHotkey.exe");
    for p in v_ahk_pid {
        let t_p = p.cmd().to_owned();
        let cmd_line = t_p.last().unwrap();
        if cmd_line.contains(&ahk) {
            return Ok(p.pid().to_string().parse::<u32>().unwrap());
        }
    }

    return Err("NONE".to_string());
}

pub fn change_signal_rgb(profile: &String) -> String {
    let profile = profile.clone();
    let mut return_string = String::new();

    if profile.is_empty() {
        return_string.push_str("No SignalRGB Profile found for {profile}");
        return return_string;
    }

    let sp = &profile;
    let mut rgb_profile = url_encode(sp.to_string());

    if rgb_profile.contains('?') {
        rgb_profile.push_str("^&-silentlaunch-");
    } else {
        rgb_profile.push_str("?-silentlaunch-");
    }

    let command_var = format!("start signalrgb://effect/apply/{}", &rgb_profile);

    let output = run_cmd(&command_var);

    if output.is_ok() {
        return_string.push_str(&format!("Changed SignalRGB to {sp}"));
    } else {
        return_string.push_str(&format!(
            "Could not execute SignalRGB Command: {}: {:?}",
            &command_var,
            &output.as_ref().unwrap()
        ));
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

    let addy = get_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        "defaults".to_string(),
        "orgb_address".to_string(),
    );
    let port = get_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        "defaults".to_string(),
        "orgb_port".to_string(),
    );
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

pub fn change_voice_attack(profile: &String) -> String {
    let profile = profile.clone();
    let mut return_string = String::new();

    if profile.is_empty() {
        return_string.push_str("No VoiceAttack profile found.");
        return return_string;
    };

    let vac = get_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        "defaults".to_string(),
        "voice_attack_path".to_string(),
    );
    let pro = (&profile).to_string();
    let cmd = format!("{} -profile {}", &vac, &pro);

    let output = Command::new(&vac)
        .creation_flags(CREATE_NO_WINDOW)
        .arg("-profile")
        .arg(&pro)
        .spawn();

    if output.is_ok() {
        return_string.push_str(&format!(
            "Changed VoiceAttack profile to {}\n\n{}",
            &profile, &cmd
        ));
    } else {
        return_string.push_str(&format!(
            "Could not change VoiceAttack profile to {}\n\n{}\nERROR:\n{:?}",
            &profile,
            &cmd,
            &output.as_ref().unwrap()
        ));
    };

    return_string
}

pub fn power_monitors(on_off: bool) -> String {
    let mut return_string = String::new();

    if on_off {
        //Turn on display
        return_string.push_str(&match send_message(
            get_by_class("Progman", None).unwrap()[0],
            WM_SYSCOMMAND,
            SC_MONITORPOWER,
            -1,
            Some(5),
        ) {
            Ok(_) => {
                reg_write_value(
                    &RegKey::predef(HKEY_LOCAL_MACHINE),
                    &Path::new("Software").join("GameMon"),
                    "display".to_string(),
                    "on".to_string(),
                )
                .unwrap();
                String::from("..OK")
            }
            Err(e) => format!("Failed to turn on monitor(s)!! || Error: {}", &e),
        });
    } else {
        //Turn off display
        return_string.push_str(&match send_message(
            get_by_class("Progman", None).unwrap()[0],
            WM_SYSCOMMAND,
            SC_MONITORPOWER,
            2,
            Some(5),
        ) {
            Ok(_) => {
                reg_write_value(
                    &RegKey::predef(HKEY_LOCAL_MACHINE),
                    &Path::new("Software").join("GameMon"),
                    "display".to_string(),
                    "off".to_string(),
                )
                .unwrap();
                String::from("..OK")
            }
            Err(e) => format!("Failed to turn off monitor(s)!! || Error: {}", &e),
        });
    }
    return_string
}

pub fn run_screensaver() -> String {
    sleep(10000);
    let ss_exe = ss_get("SCRNSAVE.EXE").to_owned();
    let mut output_str = String::new();

    if ss_exe.contains("Lively.scr") {
        output_str.push_str(&format!("Lively.scr is designated as the screensaver.  Running \"Livelycu.exe screensaver --show true\""));
        let _ = Command::new("Livelycu.exe")
            .arg("screensaver")
            .arg("--show")
            .arg("true")
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        ;
    } else {
        match get_pid(&ss_exe) {
            Ok(_) => {
                let kill = Command::new("cmd.exe")
                    .creation_flags(CREATE_NO_WINDOW)
                    .arg("/c")
                    .arg("TASKKILL")
                    .arg("/im")
                    .arg("/f")
                    .arg(&ss_exe)
                    .output();
                match kill {
                    Ok(o) => {
                        let status = o.status.code().unwrap();
                        if status == 0 {
                            output_str.push_str(
                                &format!("Screensaver detected...taking ownership of screensaver\nSUCCESS!\nstdin: TASKKILL /im /f {}\nstdout: {:?}\nstderr: {:?}\n", &ss_exe, o.stdout, o.stderr));
                        } else {
                            output_str.push_str(&format!("___Running Command___\nERROR!\nstdin: TASKKILL /im /f {}\nstdout: {:?}\nstderr: {:?}\n", &ss_exe, o.stdout, o.stderr));
                        }
                        let _ = Command::new(&ss_exe)
                            .arg("/S")
                            .creation_flags(CREATE_NO_WINDOW)
                            .output();
                    }
                    Err(e) => output_str.push_str(&format!(
                        "Could not run TASKKILL /im /f {}: {}\n",
                        &ss_exe, e
                    )),
                }
            }
            _ => {
                let run = Command::new(&ss_exe)
                    .arg("/S")
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                ;
                
                match run {
                    Ok(o) => {
                        output_str.push_str(&format!("Running Screensaver:\n\tstdin: {}\n\tstdout: {:?}\n\tstderr: {:?}\n", &ss_exe, o.stdout, o.stderr));
                    }
                    Err(e) => output_str.push_str(&format!(
                        "Could not run {}: {}\n",
                        &ss_exe, e
                    )),
                }
    
                // let mut found;
                // for _ in 0..4 {
                //     'run_ss: for _ in 0..5 {
                //         if !user_idle(){break 'run_ss};
                //         if user_idle() {
                //             found = match get_pid(&ss_exe) {
                //                 Ok(_) => true,
                //                 _ => false,
                //             };
                //             if !found {
                        
                //                 let _ = Command::new(&ss_exe)
                //                     .arg("/S")
                //                     .creation_flags(CREATE_NO_WINDOW)
                //                     .output();
                //                 output_str.push_str(&format!(
                //                     "Screensaver not found...initiating screensaver: {}\n",
                //                     &ss_exe
                //                 ));
                //             }
                //         } else {
                //             continue;
                //         }
                //     }
                // }
            }
        };
    };

    output_str
}

pub fn run_other_commands<T, U>(section_and_commands: (T, U)) -> String 
    where T: ToString, U: ToString
{

    let section = section_and_commands.0.to_string();
    let other_commands = section_and_commands.1.to_string();

    if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "last_other_commands") == section {
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
        _ => (collection.len() * 200) as u64,
    };

    let new_section = section.clone().to_owned();
    let window = get_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        section.to_string(),
        "game_or_win".to_string(),
    );

    if section.to_string() != "General" && section != "Idle" && window != "Game" {
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

    write_key(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        &"General".to_string(),
        "running_pid",
        "0",
    );

    let _oc = reg_write_value(
        &RegKey::predef(HKEY_LOCAL_MACHINE),
        &Path::new("Software").join("GameMon"),
        "last_other_commands".to_string(),
        new_section.to_string(),
    );

    let log_text = match section.as_str() {
        "General" => Arc::new(Mutex::new(String::from(format!(
            "\n_____Running General Commands_____\n"
        )))),
        "Idle" => Arc::new(Mutex::new(String::from(format!(
            "\n_____Running Idle Commands_____\n"
        )))),
        _ => Arc::new(Mutex::new(String::from(format!(
            "\n_____Running {} Commands_____\n",
            &section
        )))),
    };
    let cloned_log_text = log_text.clone();

    'command_loop: for c in collection {
        let new_sec = section.clone();
        let lt = log_text.clone();
        let newc = c.clone();
        if new_sec == "Idle" {
            if !user_idle() {
                let mut log_text = lt.lock().unwrap();
                log_text.push_str("User is no longer idle...");
                break 'command_loop;
            }
        };
        std::thread::spawn(move || {
            let mut log_text = lt.lock().unwrap();
            let output = Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/c")
                .raw_arg(&newc)
                .spawn();
            log_text.push_str(&match output {
                Ok(mut o) => {
                    let status = o.wait().unwrap().code().unwrap();
                    match status{
                        0 => format!("\n--------------------\n{}\nSUCCESS!\nstdin: {:?}\nstdout: {:?}\nstderr: {:?}\n"
                                    , &newc
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr
                                ),
                        _ => format!("\n--------------------\n{}\nERROR!\nstdin: {:?}\nstdout: {:?}\nstderr: {:?}\n"
                                    , &newc
                                    , o.stdin
                                    , o.stdout
                                    , o.stderr
                                ),
                    }
                },
                Err(e) => format!("Could not run {}: {e}\n", &newc)
            });
        });
    }

    let x = cloned_log_text.lock().unwrap().clone();
    x
}
