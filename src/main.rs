// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 01 Oct 2022 @ 15:12:38                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################
#![allow(dead_code, unused_variables, unreachable_code)]
#![cfg_attr(
    all(
      target_os = "windows",
      not(feature = "console"),
    ),
    windows_subsystem = "windows",
  )]

//   Import Data ####
// extern crate winreg;
use sysinfo::{ProcessExt, System, SystemExt};
use native_dialog::{MessageDialog, MessageType};
use ini::Ini;
use active_win_pos_rs::get_active_window;
use std::{process::{Command}, os::windows::{process::CommandExt}, io::Write, fs::OpenOptions, path::{Path, self}, cmp::Ordering};
use chrono::{Local, NaiveTime};
use reqwest::{self, header};
use {std::sync::mpsc, tray_item::TrayItem};
use winreg::enums::*;
use winreg::RegKey;
use user_idle::UserIdle;


// Environment Variables
const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

#[derive(Debug)]
enum Message {
    Quit,
}

#[derive(Debug)]
struct Instance {
    exe_name: String,
    game_window_name: String,
    name_ofahk: String,
    path_toahk: String,
    open_rgbprofile: String,
    signal_rgbprofile: String,
    voice_attack_profile: String,
    game_or_win: String,
    running_pid: String,
    running: String
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        exit_app(Some(1), "Shutdown");
    }
}

// Game Reaction
async fn game_reaction(section_name: String)-> Result<String, String>{
    let exe = match &section_name.as_str() {
        &"Screensaver" => Path::new(& screensaver().await).file_name().unwrap().to_str().unwrap().to_string(),
        _ => get_key(&section_name, "exeName").to_string()
    };

    // println!("Checking for {}", &exe);
    let game = game_process(&exe);
    
    match game{
        Ok(_) => {
            if get_key(&section_name, "running") == "True" {
                return Err(format!("{} is already running", &section_name))
            }
            write_key(&"defaults".to_string(), "gameon", "True");
            let signal_result = change_signal_rgb(&section_name);
            let open_result = change_open_rgb(&section_name);
            write_key(&section_name, "running", "True");
            write_key(&"General".to_string(), "running", "False");
            return Ok(format!("{} detected! {}. {}", &section_name, &signal_result, &open_result.await));
        }

        Err(_) => {
            if get_key(&"General".to_string(), "running") == "True"{
                return Err("General is already running".to_string());
            }
            if get_key(&section_name, "running") == "True" {
                write_key(&"defaults".to_string(), "gameon", "False");
                let general_name = &"General".to_string();
                let signal_result = change_signal_rgb(&general_name);
                let open_result = change_open_rgb(&general_name);
                write_key(&general_name, "running", "True");
                write_key(&section_name, "running", "False");
                return Ok(format!("{} no longer detected! {}. {}", &section_name, &signal_result, &open_result.await));
            };
        }
    }
    // if comp_win_pid == "" {
    //     return Err("None".to_string())
    // } else {
    //     return Ok(format!("{}", comp_win_pid));
    //     // ExitApp(Some(0));
    // }

    return Err("None".to_string())
}

// Window Reaction
async fn window_reaction(section_name: String) -> Result<String, String>{
    if get_key(&"defaults".to_string(), "gameon") == "True" {
        return Err("Gaming flag is on".to_string())
    }
    
    let exe = get_key(&section_name, "exeName");
    let win_pid = match win_pid_by_process(&exe) {
        Ok(pid) => pid,
        Err(_) => 0,
    };
    record_win_pid(&win_pid, &section_name);

async fn screensaver() -> String{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
    let screen_s: String = desktop.get_value("SCRNSAVE.EXE").unwrap();

    return screen_s;
}
// Change Signal RGB
fn change_signal_rgb(sec_name: &String) -> String{
    let sp = get_key(&sec_name, "SignalRGBprofile");
    let mut rgb_profile = url_encode(get_key(&sec_name, "SignalRGBprofile").to_string());

    if rgb_profile.contains("?"){
        rgb_profile.push_str("^&-silentlaunch-");
    } else {
        rgb_profile.push_str("?-silentlaunch-");
    }
    
    let command_var = format!("signalrgb://effect/apply/{}", &rgb_profile);
  
    let output = run_cmd(&command_var);
    let return_var: String = match output {
        Err(e) => format!("Could not execute SignalRGB Command: {}: {:?}", &command_var, e),
        Ok(_) => format!("Changed SignalRGB to {}", &sp)
    };
    
    sleep(1000);
    return return_var;
}
// Change OpenRGB
async fn change_open_rgb(sec_name: &String) -> String {
    let addy = get_key(&"defaults".to_string(), "orgb_address");
    let port = get_key(&"defaults".to_string(), "orgb_port");
    let sp = get_key(&sec_name, "OpenRGBprofile");
    let rgb_profile = url_encode(get_key(&sec_name, "OpenRGBprofile").to_string());
    let command_var = format!("http://{}:{}/{}", addy, port, &rgb_profile);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    let body = String::from("post body");


    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();

    let output = client.post(&command_var).body(body).send().await.unwrap();
    // let output = client.get(&command_var).send().await;
    

    let return_var: String = match output.status() {
        reqwest::StatusCode::OK => format!("Changed OpenRGB to {}", &sp.to_string()),
        reqwest::StatusCode::NO_CONTENT => format!("Changed OpenRGB to {}", &sp.to_string()),
        e => format!("Could not execute OpenRGB Command: {} Status: {:?}", &command_var, e)
        
    }; 
    
    return return_var.to_string();
}

fn sleep(milliseconds: u64){
    let mills = std::time::Duration::from_millis(milliseconds);
    let now = std::time::Instant::now();
    std::thread::sleep(mills);
    assert!(now.elapsed() >= mills);
}

fn exit_app(code: Option<i32>, reason: &'static str){
    match code {
        Some(i) => {
            log("w", &format!("Exiting. {} Reason: {}", reset_running(), &reason));
            std::process::exit(i);

        }
        None => {
            let reason = "Shutdown";
            log("w", &format!("Exiting. {} Reason: {}", reset_running(), &reason));
            std::process::abort();
        }
    }
}

fn reset_running() -> String{
    let path = ini_file();
    let i = Ini::load_from_file(path).unwrap();
    for (sec, _prop) in i.iter(){
        write_key(&sec.unwrap().to_string(), "running", "False");
    }
    return "Running values reset.".to_string();
}

fn initialize_log(){
    let now = timestamp();
    let mut log_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let mut log_dir: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let mut log_archive: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let filename: &str= "\\gamemon.log";
    let dirname: &str = "\\logs";
    log_file.push_str(&filename);
    log_dir.push_str(&dirname);
    log_archive.push_str(&dirname);
    log_archive.push_str("\\");
    log_archive.push_str(&now);
    log_archive.push_str(".log");
    
    let d = std::path::Path::new(&log_dir).exists();
    if d {
        let e = std::path::Path::new(&log_file).exists();
        match e {
            true => {
                std::fs::write(&log_archive, format!("{}: NEW_ARCHIVE", &now)).expect(&format!("Could not create new log archived file!! {:?}", &log_archive));
                std::fs::copy(&log_file, &log_archive).expect("Could not copy log file to archive!");
                std::fs::remove_file(&log_file).expect("Could not delete existing log!");
                std::fs::write(&log_file, format!("{}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
            false => {
                std::fs::write(&log_file, format!("{}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
        }
    } else {
        std::fs::create_dir(&log_dir).expect("Could not create logs directory!");
        let e = std::path::Path::new(&log_file).exists();
        match e {
            true => {
                std::fs::write(&log_archive, format!("{}: NEW_ARCHIVE", &now)).expect(&format!("Could not create new log archived file!! {:?}", &log_archive));
                std::fs::copy(&log_file, &log_archive).expect("Could not copy log file to archive!");
                std::fs::remove_file(&log_file).expect("Could not delete existing log!");
                std::fs::write(&log_file, format!("{}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
                
            }
            false => {
                std::fs::write(&log_file, format!("{}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
        }
    }
    
}

fn log(log_type: &'static str, log_text: &String) {
    let now = timestamp();
    let mut log_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    log_file.push_str("\\gamemon.log");
    
    if log_type == "i" {
        let data = format!("{}: INFO: {}", &now, &log_text);
        append_log(&data);
    } else if log_type == "d" {
        let data = format!("{}: DEBUG: {}", &now, &log_text);
        append_log(&data);
    } else if log_type == "w" {
        let data = format!("{}: WARNING: {}", &now, &log_text);
        append_log(&data);
    } else if log_type == "e"{
        let data = format!("{}: ERROR: {}", &now, &log_text);
        append_log(&data);
    }

}

fn append_log(data: &String){
    let mut log_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    log_file.push_str("\\gamemon.log");
    let mut lfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&log_file)
        .unwrap();
    write!(lfile, "{}", format!("{data}\n")).unwrap();
    
}

fn timestamp() -> String {
    let mut dt = Local::now().date().format("%Y%m%d").to_string();
    dt.push_str(&Local::now().time().format("%H%M%S").to_string());
    return dt
}

fn msg_box(m_title: &str, m_text: &str){
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(&m_title)
        .set_text(&format!("{}", &m_text))
        .show_alert()
        .unwrap();
}

fn debug_break(){
    log("d", &"BREAK BREAK BREAK".to_string());
}

fn record_win_pid(pid: &u32, section_header: &String){
    write_key(section_header, "running_pid", &pid.to_string());
    
}

fn get_pid(pname: Option<&str>) -> Result<String, String, String>{
    match pname {
        Some(i) => {
            
            let s = System::new_all();
            let procs = s.processes_by_exact_name(i);
            
            match Some(procs) {
                Some(p) => {
                    
                    for process in p {
                        
                        let ox = process.parent().unwrap().to_string();
                        return Ok(ox.parse::<u32>().unwrap().to_string());
                    };
 
                },
                None => {
                    return None
                }
            };
            
        },
        None => return None
    }
    return Err("NULL".to_string());
}

fn game_process(pname: &str) -> Result<String, String, String>{
    let s = System::new_all();

    for process in s.processes_by_exact_name(pname){
        return Ok(format!("{}", process.name().to_string()));

    };
    return None
}

fn ini_file() -> String{
    let mut ini_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let filename: &str= "\\gamemon.ini";
    ini_file.push_str(filename);
    return ini_file;
}

fn get_key(sec_name: &String, key_name: &'static str) -> String{
    let path = ini_file();
    let i = Ini::load_from_file(path).unwrap();
    return Ini::get_from(&i, Some(sec_name), &key_name).unwrap().to_string();
}

fn write_key(sec_name: &String, key_name: &'static str, key_value: &str){
    let path = ini_file();
    let mut i = Ini::load_from_file(path).unwrap();

    i.with_section(Some(sec_name))
        .set(key_name, key_value);
    i.write_to_file(ini_file()).unwrap();
}

fn url_encode(data: String) -> String{
    let data = data.replace("\n", "%0A");
    let data = data.replace("+", "%2b");
    let data = data.replace("\r", "%0D");
    let data = data.replace("'", "%27");
    let data = data.replace(" ", "%20");
    let data = data.replace("#", "%23");
    let data = data.replace("&", "^&");
    return data;
}

fn tray(){ 

}

fn run_cmd(cmd: &String) -> Result<std::process::Child, String, std::io::Error>{
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg(&cmd)
        .spawn();
    
    return output
}

#[tokio::main]
async fn main() {
    initialize_log();
    let _cleanup = Cleanup;
    
    let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

    tray.add_label("GameMon").unwrap();

    tray.add_menu_item("About", || {
        msg_box("About", &format!("GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language").to_string());
    })
    .unwrap();
    
    let (tx, rx) = mpsc::channel();

    tray.add_menu_item("Quit", move || {
        println!("Quit");
        tx.send(Message::Quit).unwrap();
    })
    .unwrap();
  
    // Read INI Sections and operate on each one
    loop {
        match rx.try_recv(){
            Ok(Message::Quit) => exit_app(Some(1), "Menu"),
            _ => {}
        };
        // Game and Window Reactions
        let path = ini_file();
        let i = Ini::load_from_file(path).unwrap();
        for (sect, prop) in i.iter(){
            match sect.unwrap() {
                "Screensaver" => (),
                "defaults" => (),
                sec => {
                    // capture all keys for section
                    let section = Instance {
                        exe_name: get_key(&sec.to_string(), "exeName"),
                        game_window_name: get_key(&sec.to_string(), "gameWindowName").to_string(),
                        name_ofahk: get_key(&sec.to_string(), "nameOfahk").to_string(),
                        path_toahk: get_key(&sec.to_string(), "pathToahk").to_string(),
                        open_rgbprofile: get_key(&sec.to_string(), "OpenRGBprofile").to_string(),
                        signal_rgbprofile: get_key(&sec.to_string(), "SignalRGBprofile").to_string(),
                        voice_attack_profile: get_key(&sec.to_string(), "voiceAttackProfile").to_string(),
                        game_or_win: get_key(&sec.to_string(), "game-or-win").to_string(),
                        running_pid: get_key(&sec.to_string(), "running_pid").to_string(),
                        running: get_key(&sec.to_string(), "running").to_string()
                    };

                    match section.game_or_win.as_str() {
                        "Window" => {
                            //is program running?
                            let win_bool = get_pid(Some(&section.exe_name)).expect("Cannot retrieve the PID!"); 
  
                            //is window active?

                            //change ini values

                            //change profiles

                            //run extra commands

                            //log
                            // match win_bool {
                            //     Ok(msg) => {
                            //         log("d", &format!("OK: {}",&msg.to_string()));
                            //     }
                            //     Err(emsg) => {
                            //         log("d", &format!("ERR: {}",&emsg.to_string()));
                            //     }
                            // }

                        },
                        "Game" => {

                            let ret = get_pid(Some(&section.exe_name));
                            log("d", &ret.unwrap().to_string());

                        },
                        _ => ()
                    };
                }
            };
              
        };
        
        // Idle Reaction
        let idle = idle().await;
        match idle {
            Ok(result) => {
                log("i", &format!("{}", result));
                sleep(500);   
            },
            Err(_) => continue
        };
        
        // Screensaver Reaction
        let screen_saver = game_reaction("Screensaver".to_string()).await;
        match screen_saver {
            Ok(result) => {
                log("i", &format!("{}", result));
                sleep(500);   
            },
            Err(_) => continue
        };
        sleep(500)
    }
}
