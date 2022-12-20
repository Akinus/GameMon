// Rust Programming Language
// #####################################################################
// File: ak_run.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 13:10:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Tue, 20 Dec 2022 @ 9:19:55                           #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

use std::{path::Path, process::Command, os::windows::process::CommandExt};

use mouse_rs::Mouse;
use sysinfo::{ProcessExt, System, SystemExt};
use ureq::Error;
use winapi::{um::{winbase::CREATE_NO_WINDOW, winuser::{GetForegroundWindow, SetForegroundWindow}}};

use windows_win::{raw::window::{send_message, get_by_class}};
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

use crate::{ak_utils::{url_encode, macros::{log}, sleep}, ak_io::{read::{get_value, ss_get, get_pid}, write::{reg_write_value, write_key}}};

//   Import Data ####
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
    let ahk = get_value(sec.to_string(), "path_toahk".to_string());
    
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

pub fn close_ahk(sec: &String){
    let ahk = get_value(sec.to_string(), "path_toahk".to_string());

    let binding = System::new_all();
    let v_ahk_pid = binding.processes_by_exact_name("AutoHotkey.exe");
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
    
}

pub fn change_open_rgb(profile: &String){
    if profile.is_empty() {
        log!("No OpenRGB Profile found for {profile}");
        return;
    }

    let addy = get_value("defaults".to_string(), "orgb_address".to_string());
    let port = get_value("defaults".to_string(), "orgb_port".to_string());
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
}

pub fn change_voice_attack(profile: &String){
    if profile.is_empty() {
        log!("No VoiceAttack profile found.");
        return;
    };
    
    let vac = get_value("defaults".to_string(), "voice_attack_path".to_string());
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

}

pub fn power_monitors(on_off: bool){
    if on_off {
         //Turn on display
         let mouse = Mouse::new();
         mouse.move_to(0, 0).expect("Failed to turn on monitor(s)!!");
         mouse.scroll(5).expect("Failed to scroll wheel!");
         reg_write_value(&Path::new("Software").join("GameMon"),
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
                    reg_write_value(&Path::new("Software").join("GameMon"),
                        "display".to_string(), "off".to_string()).unwrap();
                    String::from("OK")
                },
                Err(e) => log!(format!("Failed to turn off monitor(s)!! || Error: {}", &e), "e")
        };
    }
}

pub fn run_screensaver(){
    sleep(5000);
    let ss_exe = ss_get("SCRNSAVE.EXE").to_owned();
    match get_pid(Some(&ss_exe)) { // Check for Screensaver
        Ok(_) => {
            
            let _kill = match Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .arg("TASKKILL")
            .arg("/im")
            .arg("/f")
            .arg(ss_get("SCRNSAVE.EXE"))
            .spawn() {
                Ok(r) => log!(format!("Screensaver Detected...taking ownership of screensaver.\n\n{:?}", &r)),
                Err(e) => log!(format!("Failed to kill Screensaver!! Error: {}", &e), "e")
            };

            let _z = match Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .arg(ss_get("SCRNSAVE.EXE"))
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
            .arg(ss_get("SCRNSAVE.EXE"))
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
                .arg(ss_get("SCRNSAVE.EXE"))
                .arg("/S")
                .spawn() {
                    Ok(r) => log!(format!("Initiating Screensaver...\n\n{r:?}")),
                    Err(e) => log!(format!("Failed to run Screensaver!! Error: {}", &e), "e")
                };
            }
        };
    });

    take_two.join().unwrap();


}

pub fn run_other_commands(other_commands: &String){
    if other_commands.is_empty() {
        return;
    }

    let h = unsafe { GetForegroundWindow() };

    
    let mut collection = Vec::new();
    for v in other_commands.split(" && ") {
        collection.push(v.to_owned());
    }
    
    sleep(500);
    
    for c in collection {

        write_key(&"General".to_string(), "running_pid", "1");
        let output = Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/c")
            .raw_arg(&c)
            .spawn();
        match output {
            Ok(o) => log!(format!("Running {}\n\n{o:?}", &c)),
            Err(e) => log!(format!("Could not run {}: {e}", &c), "e"),
        };

    }

    sleep(3000);
    let _w = unsafe { SetForegroundWindow(h) };
    write_key(&"General".to_string(), "running_pid", "0");
    
}
