// Rust Programming Language
// #####################################################################
// File: ak_run.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 13:10:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 10 Dec 2022 @ 13:57:58                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

use std::{path::Path, process::Command, os::windows::process::CommandExt, cmp::Ordering};

use ureq::Error;
use winapi::um::winbase::CREATE_NO_WINDOW;
use windows_win::raw::window::{get_by_title, get_thread_process_id};
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

use crate::{ak_utils::{url_encode, sleep}, ak_io::read::get_value};

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
                match ahk_pid {
                    Ok(o) => {
                        let close_ahk = close_pid(o);
                        assert!(close_ahk.is_ok());
                    },
                    Err(_) => ()
                }
                
            }
        }
        
    }
    Ok(())
}

pub fn close_pid(pid: u32) -> Result<std::process::Child, std::io::Error>{
    let kill_cmd = format!("TASKKILL /PID {}", &pid);
    let output = Command::new("cmd.exe")
    .creation_flags(CREATE_NO_WINDOW)
    .arg("/c")
    .arg(&kill_cmd)
    .spawn();

    return output
}

pub fn run_cmd(cmd: &String) -> Result<std::process::Child, std::io::Error>{
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg(&cmd)
        .spawn();
    
    return output
}

pub fn get_ahk_pid(sec: &String) -> Result<u32, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let autohotkey = hklm.open_subkey("SOFTWARE\\AutoHotkey").unwrap();
    let version: String = autohotkey.get_value("Version").unwrap();
    let path = Path::new("Software").join("GameMon").join(&sec);
    let game_mon = hklm.open_subkey(&path).unwrap();
    let ahk: String = game_mon.get_value("path_toahk").unwrap();

    let title =format!("{} - AutoHotkey v{}", ahk, version);

    let find_window = get_by_title(&title, None);
    assert!(find_window.is_ok());
    
    let find_window = find_window.unwrap();
    match find_window.len().cmp(&0) {
        Ordering::Greater => {
            for w in find_window {
                let w_pid = get_thread_process_id(w);
                return Ok(w_pid.0);
            };
        },
        _ => {
            return Err("NONE".to_string())
        }
    }
    
    return Err("NONE".to_string())
}

pub fn change_signal_rgb(profile: &String) -> String{
    let sp = &profile;
    let mut rgb_profile = url_encode(sp.to_string());

    if rgb_profile.contains("?"){
        rgb_profile.push_str("^&-silentlaunch-");
    } else {
        rgb_profile.push_str("?-silentlaunch-");
    }
    
    let command_var = format!("start signalrgb://effect/apply/{}", &rgb_profile);
  
    let output = run_cmd(&command_var);
    let return_var: String = match output {
        Err(e) => format!("Could not execute SignalRGB Command: {}: {:?}", &command_var, e),
        Ok(_) => format!("Changed SignalRGB to {}", &sp)
    };
    
    sleep(1000);
    return return_var;
}

pub fn change_open_rgb(addy: &String, port: &String, profile: &String) -> Result<String, String> {
    let rgb_profile = url_encode(profile.to_string());
    let command_var = format!("http://{}:{}/{}", addy, port, &rgb_profile);

    return match ureq::post(&command_var)
        .set("User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36")
        .set("Content-Type", "application/json")
        .send_string(&format!("Requesting Change OpenRGB profile to {}", &rgb_profile)) {
            Ok(o) => Ok(format!("Changed OpenRGB to {}\n\nResponse:\nCode: {}\nContent: {}\n Url: {}",
                &rgb_profile, o.status(), o.status_text(), o.get_url())),
            Err(Error::Status(code, response)) => Err(format!("ERROR: {}", Error::Status(code, response))),
            transport => Err(format!("ERROR: {}", Error::from(transport.unwrap())))
        }

}

pub fn change_voice_attack(profile: &String) -> String {
    let vac = format!("{}", get_value("defaults".to_string(), "voice_attack_path".to_string()));
    let pro = format!("{}", &profile);
    let cmd = format!("{} -profile {}", &vac, &pro);

    let output = Command::new(&vac)
        .creation_flags(CREATE_NO_WINDOW)
        .arg("-profile")
        .arg(&pro)
        .spawn();

    return match output {
    Ok(_) => format!("Changed VoiceAttack profile to {}\n\n{}"
                , &profile
                , &cmd),
    Err(e) => format!("Could not change VoiceAttack profile to {}
                        \n\n{}\nERROR:\n{}"
                        , &profile
                        , &cmd
                        , &e)
    };
}