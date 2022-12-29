// Rust Programming Language
// #####################################################################
// File: ak_run.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 13:10:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Thu, 29 Dec 2022 @ 0:23:02                           #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

use std::{path::Path, process::Command, os::windows::process::CommandExt, cmp::Ordering, sync::{Arc, Mutex}};

use mouse_rs::Mouse;
use sysinfo::{ProcessExt, System, SystemExt};
use ureq::Error;
use winapi::{um::{winbase::CREATE_NO_WINDOW, winuser::{GetForegroundWindow, SetForegroundWindow}}};

use windows_win::{raw::window::{send_message, get_by_class}};
use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER}};

use crate::{ak_utils::{url_encode, macros::{log}, sleep, dark_hours}, ak_io::{read::{get_value, ss_get, get_pid, gamemon_value, user_idle, process_exists, window_is_active}, write::{reg_write_value, write_key}}};

//   Import Data ####
pub fn main_check(system_info: &sysinfo::System){
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let wait_time = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "exe_name".to_string());
    let night_srgb = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "night_hour_srgb_profile".to_string());
    let night_orgb = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "night_hour_orgb_profile".to_string());
    let ss_srgb = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "screensaver_srgb_profile".to_string());
    let ss_orgb = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "screensaver_orgb_profile".to_string());
    let current_profile = gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "current_profile".to_string());
    let current_priority = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), current_profile.clone(), "priority".to_string())
        .parse::<u64>().unwrap();
    let game_on = match get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), current_profile.clone(), "game_or_win".to_string()).as_str() {
        "Game" => true,
        _ => false
    };
    let display = gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "display".to_string());

    let path = Path::new("Software").join("GameMon");

    if get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "exe_name".to_string()) != ss_get(&RegKey::predef(HKEY_CURRENT_USER), "ScreenSaveTimeOut") {
        write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"Idle".to_string(), "exe_name", &ss_get(&RegKey::predef(HKEY_CURRENT_USER), "ScreenSaveTimeOut"));
    }

    if user_idle(&wait_time.parse::<u64>().unwrap() - 5)
        && current_profile == "Idle".to_string() {
            return
    }
    
    if user_idle(&wait_time.parse::<u64>().unwrap() - 5)
        && game_on == false
        && get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "running_pid".to_string()) == "0" { // PAST IDLE TIME!!!!!!
        
        let time_range = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "game_window_name".to_string());
        
        if dark_hours(&time_range) {
            log!("Idle detected during dark hours!");
            run_other_commands("Idle");
            change_open_rgb(&night_orgb);
            change_signal_rgb(&night_srgb);
            change_voice_attack(&get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "voice_attack_profile".to_string()));
            run_ahk(&"Idle".to_string());
            power_monitors(false);
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");
            let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &path
                , "current_profile".to_string()
                , "Idle".to_string());
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"Idle".to_string(), "running_pid", "1");

        } else if display == "off" && dark_hours(&time_range) == false {
            power_monitors(true);
            
        } else if get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "game_or_win".to_string()) == "Yes" {

            log!("Idle detected during daylight hours!");
            run_other_commands("Idle");
            change_open_rgb(&ss_orgb);
            change_signal_rgb(&ss_srgb);
            change_voice_attack(&get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "voice_attack_profile".to_string()));
            run_ahk(&"Idle".to_string());
            run_screensaver();
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");
            let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &path
                , "current_profile".to_string()
                , "Idle".to_string());
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"Idle".to_string(), "running_pid", "1");

        } else {
            log!("Idle detected during daylight hours!");
            run_other_commands("Idle");
            change_open_rgb(&get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "open_rgbprofile".to_string()));
            change_signal_rgb(&get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "signal_rgbprofile".to_string()));
            change_voice_attack(&get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "Idle".to_string(), "voice_attack_profile".to_string()));
            run_ahk(&"Idle".to_string());
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");
            let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &path
                , "current_profile".to_string()
                , "Idle".to_string());
            write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"Idle".to_string(), "running_pid", "1");
        };
        return
    } else if user_idle(&wait_time.parse::<u64>().unwrap() - 5) == false
        && current_profile == "Idle".to_string() {
            
        log!(format!("Idle no longer detected!"));
        power_monitors(true);
        close_ahk(&system_info,    &"Idle".to_string());
        write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"Idle".to_string(), "running_pid", "0");
        let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &path
            , "current_profile".to_string()
            , "General".to_string());
    }
    
    for sec in hklm.open_subkey(path).unwrap().enum_keys().map(|x| x.unwrap()){
        
        match sec.as_str() {
            "defaults" => continue,
            "General" => continue,
            "Idle" => continue,
            _ => (),
        };
        let section_priority = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "priority".to_owned()).parse::<u64>().unwrap();
        if current_priority.cmp(&section_priority) == Ordering::Greater {
            continue;  // DO NOTHING...section is lower priority than current priority
        };

        let exe_name = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "exe_name".to_owned());
        let open_rgbprofile = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "open_rgbprofile".to_owned());
        let signal_rgbprofile = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "signal_rgbprofile".to_owned());
        let voice_attack_profile = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "voice_attack_profile".to_owned());
                
        match get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.clone(), "game_or_win".to_owned()).as_str() {
            "Game" => {
                
                if current_profile == sec && process_exists(Some(&exe_name)){
                    continue;
                } else if current_profile != sec && process_exists(Some(&exe_name)) {
                    log!(format!("{sec} detected!"));
                    run_other_commands(&sec);
                    change_open_rgb(&open_rgbprofile);
                    change_signal_rgb(&signal_rgbprofile);
                    change_voice_attack(&voice_attack_profile);
                    run_ahk(&sec);
                    let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
                        , "current_profile".to_string()
                        , sec.clone());
                    write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");
                    close_ahk(&system_info,    &"General".to_string());
                } else if current_profile == sec && process_exists(Some(&exe_name)) == false {
                    log!(format!("{sec} no longer detected!"));
                    close_ahk(&system_info,    &sec);
                    let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
                        , "current_profile".to_string()
                        , "General".to_string());
                }
            },
            "Window" => {
                if current_profile == sec && window_is_active(Some(&exe_name)){
                    continue;
                } else if current_profile != sec && window_is_active(Some(&exe_name)) {
                    log!(format!("{sec} detected!"));
                    let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
                        , "current_profile".to_string()
                        , sec.clone());
                    run_other_commands(&sec);
                    change_open_rgb(&open_rgbprofile);
                    change_signal_rgb(&signal_rgbprofile);
                    change_voice_attack(&voice_attack_profile);
                    run_ahk(&sec);
                    write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");
                    close_ahk(&system_info,    &"General".to_string());

                } else if current_profile == sec && window_is_active(Some(&exe_name)) == false {
                    log!(format!("{sec} no longer detected!"));
                    close_ahk(&system_info,    &sec);
                    let _v = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
                        , "current_profile".to_string()
                        , "General".to_string());
                }
            },
            _ => (),
        };
        sleep(250);
    }; // End FOR loop
    
}

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

pub fn run_ahk(sec: &String){
    let ahk = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.to_string(), "path_toahk".to_string());
    
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .raw_arg(&ahk)
        .spawn();

    if output.is_ok() {
        log!(format!("{ahk} is now running!"));
    } else {
        log!(format!("Error! Could not start {ahk}\n\n{:?}", output.as_ref().unwrap()));
    };

}

pub fn close_ahk(system: &sysinfo::System, sec: &String){
    let ahk = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), sec.to_string(), "path_toahk".to_string());
    let v_ahk_pid = system.processes_by_exact_name("AutoHotkey.exe");
    for p in v_ahk_pid{
        let t_p = p.cmd().to_owned();
        let cmd_line = t_p.last().unwrap();
        if cmd_line.contains(&ahk) {
            let pid = p.pid().to_string();
            let output = Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/c")
                .arg("TASKKILL")
                .arg("/PID")
                .arg(pid)
                .spawn();
            
            if output.is_ok() {
                log!(format!("{ahk} is no longer running!"));
            } else {
                log!(format!("Error! Could not close {ahk}\n\n{:?}", output.as_ref().unwrap()));
            };
        }
    };
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

pub fn change_signal_rgb(profile: &String){
    let profile = profile.clone();
    let thread = std::thread::spawn(move ||{ 
        if profile.is_empty() {
            log!("No SignalRGB Profile found for {profile}");
            return;
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
            log!(format!("Changed SignalRGB to {sp}"));
        } else {
            log!(format!("Could not execute SignalRGB Command: {}: {:?}", &command_var, &output.as_ref().unwrap()));
        }

    });
    thread.join().unwrap();  
    
}

pub fn change_open_rgb(profile: &String){
    let profile = profile.clone();
    
    let thread = std::thread::spawn(move ||{
        if profile.is_empty() {
            log!("No OpenRGB Profile found for {profile}");
            return;
        }

        let addy = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "orgb_address".to_string());
        let port = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "defaults".to_string(), "orgb_port".to_string());
        let rgb_profile = url_encode(profile.to_string());
        let command_var = format!("http://{}:{}/{}", addy, port, &rgb_profile);
        
        log!(match ureq::post(&command_var)
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
    });
    thread.join().unwrap();
    
}

pub fn change_voice_attack(profile: &String){
    let profile = profile.clone();
    
    let thread = std::thread::spawn(move ||{

        if profile.is_empty() {
            log!("No VoiceAttack profile found.");
            return;
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
            log!(format!("Changed VoiceAttack profile to {}\n\n{}", &profile, &cmd));
        } else {
            log!(format!("Could not change VoiceAttack profile to {}\n\n{}\nERROR:\n{:?}"
                        , &profile
                        , &cmd
                        , &output.as_ref().unwrap()));
        }
    });
    thread.join().unwrap();

}

pub fn power_monitors(on_off: bool){
    if on_off {
         //Turn on display
         let mouse = Mouse::new();
         mouse.move_to(0, 0).expect("Failed to turn on monitor(s)!!");
         mouse.scroll(5).expect("Failed to scroll wheel!");
         reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon"),
             "display".to_string(), "on".to_string()).unwrap();
    } else {
        //Turn off display
        let _z = match send_message(
            get_by_class("Progman",
            None).unwrap()[0],
            0x112,
            0xF170,
            2,
            Some(5)) {
                Ok(_) => {
                    reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon"),
                        "display".to_string(), "off".to_string()).unwrap();
                    String::from("OK")
                },
                Err(e) => log!(format!("Failed to turn off monitor(s)!! || Error: {}", &e), "e")
        };
    }
}

pub fn run_screensaver(){
    let thread = std::thread::spawn(move ||{
    sleep(5000);
    let ss_exe = ss_get(&RegKey::predef(HKEY_CURRENT_USER), "SCRNSAVE.EXE").to_owned();
    match get_pid(Some(&ss_exe)) { // Check for Screensaver
        Ok(_) => {
            
            let _kill = match Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .arg("TASKKILL")
            .arg("/im")
            .arg("/f")
            .arg(ss_get(&RegKey::predef(HKEY_CURRENT_USER), "SCRNSAVE.EXE"))
            .spawn() {
                Ok(r) => log!(format!("Screensaver Detected...taking ownership of screensaver.\n\n{:?}", &r)),
                Err(e) => log!(format!("Failed to kill Screensaver!! Error: {}", &e), "e")
            };

            let _z = match Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .arg(ss_get(&RegKey::predef(HKEY_CURRENT_USER), "SCRNSAVE.EXE"))
            .arg("/S")
            .spawn() {
                Ok(r) => log!(format!("Initiating Screensaver...\n\n{r:?}")),
                Err(e) => log!(format!("Failed to run Screensaver!! Error: {}", &e), "e")
            };

        },
        _ => {
            let _z = match Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .arg(ss_get(&RegKey::predef(HKEY_CURRENT_USER), "SCRNSAVE.EXE"))
            .arg("/S")
            .spawn() {
                Ok(r) => log!(format!("Initiating Screensaver...\n\n{r:?}")),
                Err(e) => log!(format!("Failed to run Screensaver!! Error: {}", &e), "e")
            };
        }
        };

        let take_two = std::thread::spawn(move ||{
            sleep(5000);
            match get_pid(Some(&ss_exe)) { // Check for Screensaver
                Ok(_) => return,
                _ => {
                    let _z = match Command::new("cmd.exe")
                    .creation_flags(CREATE_NO_WINDOW)
                    .arg("/c")
                    .arg(ss_get(&RegKey::predef(HKEY_CURRENT_USER), "SCRNSAVE.EXE"))
                    .arg("/S")
                    .spawn() {
                        Ok(r) => log!(format!("Initiating Screensaver...\n\n{r:?}")),
                        Err(e) => log!(format!("Failed to run Screensaver!! Error: {}", &e), "e")
                    };
                }
            };
        });

        take_two.join().unwrap();
    });

    thread.join().unwrap();


}

pub fn run_other_commands(section: &str){
    if gamemon_value(&RegKey::predef(HKEY_LOCAL_MACHINE), "last_other_commands".to_owned()) == section {
        return
    };
    
    let other_commands = get_value(&RegKey::predef(HKEY_LOCAL_MACHINE), section.to_string(), "other_commands".to_string());

    if other_commands.is_empty() {
        return;
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
    std::thread::spawn(move || {
        let h = unsafe { GetForegroundWindow() };

        for _ in 0..wait_time {
            let z = unsafe { GetForegroundWindow() };
            if z != h {
                let _w = unsafe { SetForegroundWindow(h) };
            }
            sleep(1);
        }

        write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "0");

        let _oc = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
            , "last_other_commands".to_string()
            , new_section.to_string());
    });

    let counter = Arc::new(Mutex::new(0));
    
    for c in collection {

        write_key(&RegKey::predef(HKEY_LOCAL_MACHINE), &"General".to_string(), "running_pid", "1");
        
        let wait_time = Arc::clone(&counter);

        std::thread::spawn(move || {
            sleep(*wait_time.lock().unwrap());
            let output = Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/c")
                .raw_arg(&c)
                .spawn();
            match output {
                Ok(o) => log!(format!("Running {}\n\n{o:?}", &c)),
                Err(e) => log!(format!("Could not run {}: {e}", &c), "e"),
            };
            *wait_time.lock().unwrap() += 250;
        });
        
    }

}
