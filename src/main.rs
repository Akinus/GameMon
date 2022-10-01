// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 01 Oct 2022 @ 11:10:35                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

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
    exeName: String,
    gameWindowName: String,
    nameOfahk: String,
    pathToahk: String,
    OpenRGBprofile: String,
    SignalRGBprofile: String,
    voiceAttackProfile: String,
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
        _ => get_key(&section_name, "exeName")
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

    
    // println!("{:?}", exe_name);
    let fore_win_pid: String = get_active_window().unwrap().process_id.to_string();
    let comp_win_pid = get_key(&section_name, "running_pid");
    
    if fore_win_pid == comp_win_pid {
        if get_key(&section_name, "running") == "True" {
            return Err(format!("{} is already running", &section_name))
        }
        let signal_result = change_signal_rgb(&section_name);
        let open_result = change_open_rgb(&section_name);
        write_key(&section_name, "running", "True");
        write_key(&"General".to_string(), "running", "False");
        return Ok(format!("{} detected! {}. {}", &section_name, &signal_result, &open_result.await));
    } else {
        if get_key(&"General".to_string(), "running") == "True"{
            return Err("General is already running".to_string());
        }
        if get_key(&section_name, "running") == "True" {
            let general_name = &"General".to_string();
            let signal_result = change_signal_rgb(&general_name);
            let open_result = change_open_rgb(&general_name);
            write_key(&general_name, "running", "True");
            write_key(&section_name, "running", "False");
            // game_reaction("Screensaver".to_string()).await.unwrap();
            return Ok(format!("{} no longer detected! {}. {}", &section_name, &signal_result, &open_result.await));
        };
        return Err("None".to_string())
    }
    // println!("Foreground window is: {:?}\nCompare window is: {:?}", fore_win_pid, comp_win_pid);
}

// Idle Reaction
async fn idle() -> Result<String, String>{
    let idle_wait = get_key(&"Idle".to_string(), "exeName");
    let idle_time = UserIdle::get_time().unwrap();
    let idle_seconds = idle_time.as_seconds();
    let section_name: String = "Idle".to_string();
    let time_of_day = Local::now().time();
    let time_string = get_key(&"Idle".to_string(), "gameWindowName");
    let time_range = time_string.split("-").collect::<Vec<&str>>();
    let start_time = NaiveTime::parse_from_str(time_range[0], "%H%M").unwrap();
    let end_time = NaiveTime::parse_from_str(time_range[1], "%H%M").unwrap();
    // log("d", &format!("Start Time: {:?}. End Time: {:?}. Time Now: {}", start_time, end_time, time_of_day));
    // ExitApp(Some(1), "Test");

    match idle_seconds.cmp(&idle_wait.parse::<u64>().unwrap()){
        Ordering::Greater => {
            if get_key(&section_name, "running") == "True" {
                return Err("Idle is already running".to_string());
            }
            write_key(&"defaults".to_string(), "gameon", "True");
            write_key(&section_name, "running", "True");
            write_key(&"General".to_string(), "running", "False");
            
            if (time_of_day > start_time) && (time_of_day < end_time) {
                let signal_result = change_signal_rgb(&section_name);
                let open_result = change_open_rgb(&section_name);
               
                return Ok(format!("{} detected during night hours! {}. {}", &section_name, &signal_result, &open_result.await));
            } else {
                let ss = screensaver().await;
                Command::new("cmd.exe")
                .creation_flags(CREATE_NO_WINDOW)
                .arg("/C")
                .arg(&ss)
                .arg("/S")
                .spawn().unwrap();
                return Ok(format!("{} detected during day hours! Running Screensaver. ", &section_name));
            };

        },
        _ => {
            if get_key(&"General".to_string(), "running") == "True"{
                return Err("General is already running".to_string());
            };
            write_key(&"defaults".to_string(), "gameon", "False");
            if get_key(&section_name, "running") == "True" {
                let general_name = &"General".to_string();
                let signal_result = change_signal_rgb(&general_name);
                let open_result = change_open_rgb(&general_name);
                write_key(&general_name, "running", "True");
                write_key(&section_name, "running", "False");
                write_key(&"Screensaver".to_string(), "running", "False");
                return Ok(format!("{} no longer detected! {}. {}", &section_name, &signal_result, &open_result.await));
            };   
        }
    };  

    return Err("None".to_string())
}

// Screensaver reaction
async fn screensaver() -> String{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
    let screen_s: String = desktop.get_value("SCRNSAVE.EXE").unwrap();

    return screen_s;
}
// Change Signal RGB
fn change_signal_rgb(sec_name: &String) -> String{
    let sp = get_key(&sec_name, "SignalRGBprofile");
    let mut rgb_profile = url_encode(get_key(&sec_name, "SignalRGBprofile"));

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
    let rgb_profile = url_encode(get_key(&sec_name, "OpenRGBprofile"));
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

// Run Extra Commands

// GUI

// Extra Functions
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
                std::fs::write(&log_archive, format!("{:?}: NEW_ARCHIVE", &now)).expect(&format!("Could not create new log archived file!! {:?}", &log_archive));
                std::fs::copy(&log_file, &log_archive).expect("Could not copy log file to archive!");
                std::fs::remove_file(&log_file).expect("Could not delete existing log!");
                std::fs::write(&log_file, format!("{:?}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
            false => {
                std::fs::write(&log_file, format!("{:?}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
        }
    } else {
        std::fs::create_dir(&log_dir).expect("Could not create logs directory!");
        let e = std::path::Path::new(&log_file).exists();
        match e {
            true => {
                std::fs::write(&log_archive, format!("{:?}: NEW_ARCHIVE", &now)).expect(&format!("Could not create new log archived file!! {:?}", &log_archive));
                std::fs::copy(&log_file, &log_archive).expect("Could not copy log file to archive!");
                std::fs::remove_file(&log_file).expect("Could not delete existing log!");
                std::fs::write(&log_file, format!("{:?}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
                
            }
            false => {
                std::fs::write(&log_file, format!("{:?}: INFO: Log Initialized. GameMon started...\n", &now)).expect("Could not create new log file!!");
            }
        }
    }
    
}

fn log(log_type: &'static str, log_text: &String) {
    let now = timestamp();
    let mut log_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    log_file.push_str("\\gamemon.log");
    
    if log_type == "i" {
        let data = format!("{:#?}: INFO: {:#?}", &now, &log_text);
        append_log(&data);
    } else if log_type == "d" {
        let data = format!("{:#?}: DEBUG: {:#?}", &now, &log_text);
        append_log(&data);
    } else if log_type == "w" {
        let data = format!("{:#?}: WARNING: {:#?}", &now, &log_text);
        append_log(&data);
    } else if log_type == "e"{
        let data = format!("{:#?}: ERROR: {:#?}", &now, &log_text);
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
    if let Err(e) = writeln!(lfile, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    };
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

fn win_pid_by_process(pname: &str) ->  Result<u32, String>{
    let run_pid =get_pid(Some(pname)).unwrap();
    
    if &run_pid == "()" {
        log("d", &format!("PID: {} found for {}", &run_pid, &pname).to_string());
        return Err(format!("No process found with name: {}", &pname));
    } else {
        return Ok(run_pid.parse::<u32>().unwrap());
    }

    return Err("None".to_string());
}

fn record_win_pid(pid: &u32, section_header: &String){
    write_key(section_header, "running_pid", &pid.to_string());
    
}

fn get_pid(pname: Option<&str>) -> Result<String, String>{
    match pname {
        Some(i) => {
            let s = System::new_all(); 
            for process in s.process_by_exact_name(i) {
                
                let ox = process.parent().unwrap().to_string();
                return Ok(ox.parse::<u32>().unwrap().to_string());
            };
        },
        None => return Err("NULL".to_string())
    }
    return Err("NULL".to_string());
}

fn game_process(pname: &str) -> Result<String, String>{
    let s = System::new_all();

    for process in s.processes_by_exact_name(pname){
        return Ok(format!("{}", process.name().to_string()));

    };
    return Err("None".to_string());
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

fn run_cmd(cmd: &String) -> Result<std::process::Child, std::io::Error>{
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg("start")
        .arg(&cmd)
        .spawn();
    
    return output
}

#[tokio::main]
async fn main() {
    initialize_log();
    let _cleanup = Cleanup;
    
    // let window_msg = windows_win::raw::window::send_message(unsafe { GetDesktopWindow() }, 0x0112, 0xF170, 2, Some(5));
    // match window_msg {
    //     Ok(o) => ExitApp(Some(1), "Ok" ),
    //     Err(e) => ExitApp(Some(1), "Error" ),
    // };


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
    loop{
        match rx.try_recv(){
            Ok(Message::Quit) => exit_app(Some(1), "Menu"),
            _ => {}
        };
        // Game and Window Reactions
        let path = ini_file();
        let i = Ini::load_from_file(path).unwrap();
        for (sect, prop) in i.iter(){
            match sect.unwrap() {
                "Screensaver" => continue,
                "Idle" => continue,
                "defaults" => continue,
                sec => {
                    // capture all keys for section
                    let section = Instance {
                        exeName: get_key(&sec.to_string(), "exeName"),
                        gameWindowName: get_key(&sec.to_string(), "gameWindowName"),
                        nameOfahk: get_key(&sec.to_string(), "nameOfahk"),
                        pathToahk: get_key(&sec.to_string(), "pathToahk"),
                        OpenRGBprofile: get_key(&sec.to_string(), "OpenRGBprofile"),
                        SignalRGBprofile: get_key(&sec.to_string(), "SignalRGBprofile"),
                        voiceAttackProfile: get_key(&sec.to_string(), "voiceAttackProfile"),
                        game_or_win: get_key(&sec.to_string(), "game-or-win"),
                        running_pid: get_key(&sec.to_string(), "running_pid"),
                        running: get_key(&sec.to_string(), "running")
                    };

                    match section.game_or_win.as_str() {
                        "Window" => {
                            //is program running?
                            let ret = get_pid(Some(&section.exeName));
                            log("d", &ret.unwrap());
                            exit_app(Some(1), "test");
                            //is window active?

                            //change ini values

                            //change profiles

                            //run extra commands

                            //log


                            // let win = window_reaction(sec.unwrap().to_string()).await;
                            // match win {
                            //     Ok(result) => {
                            //         log("i", &format!("{}", result));
                            //         sleep(500);   
                            //     },
                            //     Err(_) => continue
                            // };
                        },
                        "Game" => {
                            // let game = game_reaction(sec.unwrap().to_string()).await;
                            // match game {
                            //     Ok(result) => {
                            //         log("i", &format!("{}", result));
                            //         sleep(500);   
                            //     },
                            //     Err(_) => continue
                            // };
                        },
                        _ => continue
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
    // exit_app(Some(0), &"Loop broken, shutting down.");
}
