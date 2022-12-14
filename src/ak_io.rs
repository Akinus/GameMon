// Rust Programming Language
// #####################################################################
// File: ak_io.rs                                                      #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:39:37                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Wed, 28 Dec 2022 @ 21:00:02                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
pub mod read {
    use std::{path::Path, cmp::Ordering};
    use sysinfo::{System, SystemExt, Pid, ProcessExt};
    use winapi::{um::winuser::{LASTINPUTINFO, PLASTINPUTINFO, GetLastInputInfo}};
    use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, RegDisposition}};
    use active_win_pos_rs::get_active_window;

    use crate::{ak_utils::macros::
    {
        d_quote,
        log
    }, ak_io::write::{write_key, reg_section_new, reg_write_value}};

    pub fn get_pid(pname: Option<&str>) -> Result<u32, &str>{
        let mut pids = Vec::new();
        if pname.is_none() {
            return Err(&"No Match Found");
        };

        let i = pname.unwrap();

        let s = System::new_all();
        let procs = s.processes_by_exact_name(i);

        for process in procs {
            let ox = process.parent().unwrap().to_string();
            if ox == "0" {
                continue;
            } else {
                pids.push(ox.parse::<u32>().unwrap());
            }
        };

        let r = match pids.is_empty() {
            true => Err("Program Not Found!"),
            false => Ok(pids.last().unwrap().to_owned()),
        };

        return r;

    }

    pub fn process_exists(pname: Option<&str>) -> bool {
        let mut pids = Vec::new();
        if pname.is_none() {
            return false;
        };

        let i = pname.unwrap();

        let s = System::new_all();
        let procs = s.processes_by_exact_name(i);

        for process in procs {
            let ox = process.parent().unwrap().to_string();
            if ox == "0" {
                continue;
            } else {
                pids.push(ox.parse::<u32>().unwrap());
            }
        };

        let r = match pids.is_empty() {
            true => false,
            false => true,
        };

        return r
    }

    pub fn window_is_active(process_name: Option<&str>) -> bool {
        let active_pid = get_active_window().unwrap().process_id;

        let s = System::new_all();
        let process = s.process(Pid::from(active_pid as usize)).unwrap().name();
           
        if process == process_name.unwrap() {
            return true;
        } else {
            return false;
        } 
    }

    pub struct Instance {
        pub exe_name: String,
        pub game_window_name: String,
        pub name_ofahk: String,
        pub path_toahk: String,
        pub open_rgbprofile: String,
        pub signal_rgbprofile: String,
        pub voice_attack_profile: String,
        pub game_or_win: String,
        pub running: String,
        pub running_pid: String,
        pub other_commands: String,
        pub priority: String
    }

    impl Instance {
        pub fn new() -> Instance {
            return Instance {
                exe_name: "".to_string(),
                game_window_name: "".to_string(),
                name_ofahk: "".to_string(),
                path_toahk: "".to_string(),
                open_rgbprofile: "".to_string(),
                signal_rgbprofile: "".to_string(),
                voice_attack_profile: "".to_string(),
                game_or_win: "".to_string(),
                running: "".to_string(),
                running_pid: "".to_string(),
                other_commands: "".to_string(),
                priority: "".to_string()
            }
        }
    }

    impl Drop for Instance{
        fn drop(&mut self) {
        }
    }

    pub struct Defaults {
        pub openrgb_path: String,
        pub exit_reason: String,
        pub voice_attack_path: String,
        pub default_orgb_profile: String,
        pub default_srgb_profile: String,
        pub screensaver_orgb_profile: String,
        pub screensaver_srgb_profile: String,
        pub night_hour_orgb_profile: String,
        pub night_hour_srgb_profile: String,
        pub orgb_port: String,
        pub orgb_address: String,
        pub gameon: String,
        pub window_flag: String,
        pub current_priority: String
    }
    
    impl Defaults {
        pub fn new() -> Defaults {
            return Defaults {
                openrgb_path: "".to_string(),
                exit_reason: "".to_string(),
                voice_attack_path: "".to_string(),
                default_orgb_profile: "".to_string(),
                default_srgb_profile: "".to_string(),
                screensaver_orgb_profile: "".to_string(),
                screensaver_srgb_profile: "".to_string(),
                night_hour_orgb_profile: "".to_string(),
                night_hour_srgb_profile: "".to_string(),
                orgb_port: "".to_string(),
                orgb_address: "".to_string(),
                gameon: "".to_string(),
                window_flag: "".to_string(),
                current_priority: "".to_string()
            }
        }
    }

    pub fn get_defaults() -> Defaults {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        path = Path::new("Software").join("GameMon").join("defaults");
        let sec = hklm.open_subkey(&path).unwrap();
        let mut defaults = Defaults::new();
        
    
        for i in gamemon.enum_keys().map(|x| x.unwrap()){
            match i.as_str() {
                "defaults" => {
                    for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
                        match name.as_str() {
                            "openrgb_path" => defaults.openrgb_path = d_quote!(&value.to_string()),
                            "exit_reason" => defaults.exit_reason = d_quote!(&value.to_string()),
                            "voice_attack_path" => defaults.voice_attack_path = d_quote!(&value.to_string()),
                            "default_orgb_profile" => defaults.default_orgb_profile = d_quote!(&value.to_string()),
                            "default_srgb_profile" => defaults.default_srgb_profile = d_quote!(&value.to_string()),
                            "screensaver_orgb_profile" => defaults.screensaver_orgb_profile = d_quote!(&value.to_string()),
                            "screensaver_srgb_profile" => defaults.screensaver_srgb_profile = d_quote!(&value.to_string()),
                            "night_hour_orgb_profile" => defaults.night_hour_orgb_profile = d_quote!(&value.to_string()),
                            "night_hour_srgb_profile" => defaults.night_hour_srgb_profile = d_quote!(&value.to_string()),
                            "orgb_port" => defaults.orgb_port = d_quote!(&value.to_string()),
                            "orgb_address" => defaults.orgb_address = d_quote!(&value.to_string()),
                            "gameon" => defaults.gameon = d_quote!(&value.to_string()),
                            "window_flag" => defaults.window_flag = d_quote!(&value.to_string()),
                            "current_priority" => defaults.current_priority = d_quote!(&value.to_string()),
                            
                            _ => ()
                        }
                    };
                }
                _ => ()
            }
        }
        
        return defaults
     
    }

    pub fn get_section(sec_name: &String) -> Instance {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        path = Path::new("Software").join("GameMon").join(&sec_name);
        let sec = hklm.open_subkey(&path).unwrap();
        let mut section = Instance::new();
    
        for i in gamemon.enum_keys().map(|x| x.unwrap()){
            if &i == sec_name {
                for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
                    match name.as_str() {
                        "exe_name" => section.exe_name = d_quote!(&value.to_string()),
                        "game_window_name" => section.game_window_name = d_quote!(&value.to_string()),
                        "name_ofahk" => section.name_ofahk = d_quote!(&value.to_string()),
                        "path_toahk" => section.path_toahk = sec.get_value("path_toahk").unwrap(),
                        "open_rgbprofile" => section.open_rgbprofile = d_quote!(&value.to_string()),
                        "signal_rgbprofile" => section.signal_rgbprofile = d_quote!(&value.to_string()),
                        "voice_attack_profile" => section.voice_attack_profile = d_quote!(&value.to_string()),
                        "game_or_win" => section.game_or_win = d_quote!(&value.to_string()),
                        "running" => section.running = d_quote!(&value.to_string()),
                        "running_pid" => section.running_pid = d_quote!(&value.to_string()),
                        "other_commands" => section.other_commands = sec.get_value("other_commands").unwrap(),
                        "priority" => section.priority = d_quote!(&value.to_string()),
                        _ => ()
                    }
                }
            }
           
        }
    
        return section
    }

    pub fn ss_get(hkey: &RegKey, key_name: &'static str) -> String{
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
        let screen_s: String = desktop.get_value(&key_name).unwrap();
    
        return screen_s;
    }
    
    pub fn get_value(hkey: &RegKey, section: String, key: String) -> String{
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(section);
        let gamemon = hklm.open_subkey(&path).unwrap();
        gamemon.get_value(key).unwrap()
    }

    pub fn gamemon_value(hkey: &RegKey, key: String) -> String{
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        gamemon.get_value(key).unwrap()
    }

    pub fn reg_check(hkey: &RegKey){
        let hklm = hkey;
        let mut path = Path::new("Software").join("GameMon");
        let disp = hklm.create_subkey(&path).unwrap().1;
    
        match disp {
            RegDisposition::REG_CREATED_NEW_KEY => {
                log!(format!("A new key has been created at {:?}", &path));
                let ini_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
                match reg_write_value(&hkey, &path, "InstallDir".to_string(), format!("{}", &ini_file)) {
                    Ok(_) => {
                        log!(&format!("Wrote value {:?} to {}\\InstallDir", &path, &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value {:?} to {}\\InstallDir", &path, &ini_file), "e");
                    },
                };
                match reg_write_value(&hkey, &path, "display".to_string(), (&"on").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value {:?} to {}\\display", &path, &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value {:?} to {}\\display", &path, &ini_file), "e");
                    },
                };
                match reg_write_value(&hkey, &path, "current_profile".to_string(), (&"General").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value \"General\" to {}\\current_profile", &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value \"General\" to {}\\current_profile", &ini_file), "e");
                    },
                };
                match reg_write_value(&hkey, &path, "last_other_commands".to_string(), (&"General").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value \"General\" to {}\\last_other_commands", &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value \"General\" to {}\\last_other_commands", &ini_file), "e");
                    },
                };
    
                for i in ["General", "Idle", "defaults"] {
                    path = Path::new("Software").join("GameMon").join(&i);
                    match &i {
                        &"defaults" => {
                            let disp = hklm.create_subkey(&path).unwrap().1;
            
                            match disp {
                                RegDisposition::REG_CREATED_NEW_KEY => {
                                    log!(format!("A new section has been created at {:?}", &path));
                                    
                                    for i in ["openrgb_path".to_string(),
                                    "exit_reason".to_string(),
                                    "voice_attack_path".to_string(),
                                    "default_orgb_profile".to_string(),
                                    "default_srgb_profile".to_string(),
                                    "screensaver_orgb_profile".to_string(),
                                    "screensaver_srgb_profile".to_string(),
                                    "night_hour_orgb_profile".to_string(),
                                    "night_hour_srgb_profile".to_string(),
                                    "orgb_port".to_string(),
                                    "orgb_address".to_string(),
                                    "gameon".to_string(),
                                    "window_flag".to_string()] {
                                        match reg_write_value(&hkey, &path, String::from(&i), "".to_string()) {
                                            Ok(_) => {
                                                log!(&format!("Created empty value {}", &i));
                                            },
                                            Err(_) => {
                                                log!(&format!("Could not write value {} to {:?}", &i, &path), "e");
                                            },
                                        };
                                    }
                                    let section_name = "defaults".to_string();
                                    write_key(&hkey, &section_name, "exit_reason", "");
                                    write_key(&hkey, &section_name, "pathToSchemas", "");
                                    write_key(&hkey, &section_name, "orgb_port", "6742");
                                    write_key(&hkey, &section_name, "orgb_address", "127.0.0.1");
                                    write_key(&hkey, &section_name, "gameon", "False");
                                    write_key(&hkey, &section_name, "current_priority", "0");
                                    write_key(&hkey, &section_name, "running", "");
                                    write_key(&hkey, &section_name, "window_flag", "General");
                                    write_key(&hkey, &section_name, "screensaver_orgb_profile", "General");
                                    write_key(&hkey, &section_name, "screensaver_srgb_profile", "Screen Ambience");
                                },
                                RegDisposition::REG_OPENED_EXISTING_KEY => {
                                    log!(&"An existing key has been opened".to_string());
                                },
                            }
                        },
                        o => {
                            reg_section_new(&hkey, o.to_string())
                        }
                    }
                    
                }
            
                let mut section_name = "General".to_string();
                write_key(&hkey, &section_name, "OpenRGBprofile", "General");
                write_key(&hkey, &section_name, "SignalRGBprofile", "General");
                write_key(&hkey, &section_name, "game-or-win", "Game");
                write_key(&hkey, &section_name, "priority", "0");
            
                section_name = "Idle".to_string();
                write_key(&hkey, &section_name, "exeName", "300");
                write_key(&hkey, &section_name, "gameWindowName", "2100-0600");
                write_key(&hkey, &section_name, "game-or-win", "Game");
                write_key(&hkey, &section_name, "priority", "4");
            },
            RegDisposition::REG_OPENED_EXISTING_KEY => {
                log!(&"An existing key has been opened".to_string());
            },
        }
    
        
    
        
    }

    pub fn user_idle(wait_time: u64) -> bool {
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
            false => Err("GetLastInputInfo failed".to_string())
        }.unwrap().as_secs();

        if idle_seconds.cmp(&(&wait_time)) == Ordering::Greater {
            return true
        } else {
            return false
        }
    }

}

pub mod write {
    use std::path::{Path, PathBuf};
    use crate::ak_utils::macros::{
        log
    };

    use winreg::{RegKey, enums::{RegDisposition::{REG_CREATED_NEW_KEY, REG_OPENED_EXISTING_KEY}}};


    pub fn write_key(hkey: &RegKey, sec_name: &String, key_name: &'static str, key_value: &str){
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(&sec_name);
        let key = hklm.create_subkey(&path).unwrap().0;
    
        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn delete_section(hkey: &RegKey, sec_name: &String){
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(sec_name);
        hklm.delete_subkey_all(path).unwrap();
    }

    pub fn ss_set(hkey: &RegKey, key_name: &'static str, key_value: &str){
        let hkcu = hkey;
        let path = Path::new("Control Panel").join("Desktop");
        let key = hkcu.create_subkey(&path).unwrap().0;
    
        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn reset_running(hkey: &RegKey){
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon");
        let game_mon = hklm.open_subkey(&path).unwrap();
        
        for sec in game_mon.enum_keys().map(|x| x.unwrap()){
            match &sec.as_str() {
                &"General" => (),
                &"defaults" => (),
                _ => {
                    write_key(&hkey, &sec, "running", "False");
                    write_key(&hkey, &sec, "running_pid", "0");
                }
            }
        }
    
        write_key(&hkey, &"defaults".to_string(), "gameon", "False");
        write_key(&hkey, &"General".to_string(), "running", "True");
        write_key(&hkey, &"General".to_string(), "running_pid", "0");
        let _v = reg_write_value(&hkey, &Path::new("Software").join("GameMon")
            , "current_profile".to_string()
            , "General".to_string());
        log!("Running values reset.".to_string(), "w");
    }

    pub fn write_section(hkey: &RegKey, sec_name: &String){
        write_key(&hkey, sec_name, "exeName", "");
        write_key(&hkey, sec_name, "gameWindowName", "");
        write_key(&hkey, sec_name, "nameOfahk", "");
        write_key(&hkey, sec_name, "pathToahk", "");
        write_key(&hkey, sec_name, "OpenRGBprofile", "");
        write_key(&hkey, sec_name, "voiceAttackProfile", "");
        write_key(&hkey, sec_name, "SignalRGBprofile", "");
        write_key(&hkey, sec_name, "game-or-win", "");
        write_key(&hkey, sec_name, "priority", "");
        write_key(&hkey, sec_name, "running", "");
        write_key(&hkey, sec_name, "running_pid", "");
    }

    pub fn reg_write_value(hkey: &RegKey, path: &PathBuf, name: String, value: String) -> Result<(), std::io::Error> {
        let hklm = hkey;
        let key = hklm.create_subkey(&path).unwrap().0;
    
        return key.set_value(&name, &value);
    }

    pub fn reg_section_new(hkey: &RegKey, sec: String) {
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(&sec);
        let disp = hklm.create_subkey(&path).unwrap().1;
    
        match disp {
            REG_CREATED_NEW_KEY => {
                log!(format!("A new section has been created at {:?}", &path));
                for i in ["exe_name".to_string(),
                "game_window_name".to_string(),
                "name_ofahk".to_string(),
                "path_toahk".to_string(),
                "open_rgbprofile".to_string(),
                "signal_rgbprofile".to_string(),
                "voice_attack_profile".to_string(),
                "game_or_win".to_string(),
                "running".to_string(),
                "running_pid".to_string(),
                "other_commands".to_string()] {
                    match reg_write_value(&hkey, &path, String::from(&i), "".to_string()) {
                        Ok(_) => {
                            log!(&format!("Created empty value {}", &i));
                        },
                        Err(_) => {
                            log!(&format!("Could not write value {} to {:?}", &i, &path), "e");
                        },
                    };
                }
            },
            REG_OPENED_EXISTING_KEY => ()
        }
    }

}

pub mod logging {
    use winreg::{RegKey};
    use crate::{ak_utils::macros::{
        log
    }, ak_io::read::get_value};

    
    pub fn initialize_log(hkey: &RegKey){
        eventlog::register("GameMon Log").unwrap();
        eventlog::init("GameMon Log", log::Level::Trace).unwrap();
        
        let hklm = hkey;
        let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
        let mut script_dir: String = g_key.get_value("InstallDir").unwrap();
    
        let script_dirname: &str = "\\scripts";
        script_dir.push_str(&script_dirname);
        
        let s = std::path::Path::new(&script_dir).exists();
        if s {
            
        } else {
            std::fs::create_dir(&script_dir).expect("Could not create scripts directory!");
        }
    
        let mut custom_view = "C:\\ProgramData\\Microsoft\\Event Viewer\\Views\\gamemon_trace_logs.xml";
        let e = std::path::Path::new(&custom_view).exists();
        match e {
            true => (),
            false => {
                std::fs::write(&custom_view, "<ViewerConfig><QueryConfig><QueryParams><Simple>
                <Channel>Application</Channel><EventId>4</EventId><Source>GameMon Log</Source>
                <RelativeTimeInfo>0</RelativeTimeInfo><BySource>False</BySource></Simple></QueryParams>
                <QueryNode><Name LanguageNeutralValue=\"GameMon Trace Logs\">GameMon Trace Logs</Name>
                <Description>Trace logs from GameMon.exe</Description>
                <QueryList><Query Id=\"0\" Path=\"Application\">
                <Select Path=\"Application\">*[System[Provider[@Name='GameMon Log'] 
                and (EventID=4)]]</Select></Query></QueryList></QueryNode></QueryConfig></ViewerConfig>
                ").expect("Could not create new event viewer custom view for Trace Logs!!");
            }
        };
    
        custom_view = "C:\\ProgramData\\Microsoft\\Event Viewer\\Views\\gamemon_logs.xml";
        let e = std::path::Path::new(&custom_view).exists();
        match e {
            true => (),
            false => {
                std::fs::write(&custom_view, "<ViewerConfig><QueryConfig><QueryParams><Simple><Channel>Application</Channel>
                <EventId>1-3</EventId><Source>GameMon Log</Source>
                <RelativeTimeInfo>0</RelativeTimeInfo>
                <BySource>False</BySource></Simple></QueryParams>
                <QueryNode><Name LanguageNeutralValue=\"GameMon Logs\">GameMon Logs</Name>
                <Description>Events logged from GameMon</Description>
                <QueryList><Query Id=\"0\" Path=\"Application\">
                <Select Path=\"Application\">*[System[Provider[@Name='GameMon Log'] 
                and ( (EventID &gt;= 1 and EventID &lt;= 3) )]]</Select></Query></QueryList></QueryNode></QueryConfig>
                <ResultsConfig><Columns><Column Name=\"Level\" Type=\"System.String\" Path=\"Event/System/Level\" Visible=\"\">160</Column>
                <Column Name=\"Keywords\" Type=\"System.String\" Path=\"Event/System/Keywords\">70</Column>
                <Column Name=\"Date and Time\" Type=\"System.DateTime\" Path=\"Event/System/TimeCreated/@SystemTime\" Visible=\"\">210</Column>
                <Column Name=\"Source\" Type=\"System.String\" Path=\"Event/System/Provider/@Name\" Visible=\"\">120</Column>
                <Column Name=\"Event ID\" Type=\"System.UInt32\" Path=\"Event/System/EventID\" Visible=\"\">120</Column>
                <Column Name=\"Task Category\" Type=\"System.String\" Path=\"Event/System/Task\" Visible=\"\">123</Column>
                <Column Name=\"User\" Type=\"System.String\" Path=\"Event/System/Security/@UserID\">50</Column>
                <Column Name=\"Operational Code\" Type=\"System.String\" Path=\"Event/System/Opcode\">110</Column>
                <Column Name=\"Log\" Type=\"System.String\" Path=\"Event/System/Channel\">80</Column>
                <Column Name=\"Computer\" Type=\"System.String\" Path=\"Event/System/Computer\">170</Column>
                <Column Name=\"Process ID\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@ProcessID\">70</Column>
                <Column Name=\"Thread ID\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@ThreadID\">70</Column>
                <Column Name=\"Processor ID\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@ProcessorID\">90</Column>
                <Column Name=\"Session ID\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@SessionID\">70</Column>
                <Column Name=\"Kernel Time\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@KernelTime\">80</Column>
                <Column Name=\"User Time\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@UserTime\">70</Column>
                <Column Name=\"Processor Time\" Type=\"System.UInt32\" Path=\"Event/System/Execution/@ProcessorTime\">100</Column>
                <Column Name=\"Correlation Id\" Type=\"System.Guid\" Path=\"Event/System/Correlation/@ActivityID\">85</Column>
                <Column Name=\"Relative Correlation Id\" Type=\"System.Guid\" Path=\"Event/System/Correlation/@RelatedActivityID\">140</Column>
                <Column Name=\"Event Source Name\" Type=\"System.String\" Path=\"Event/System/Provider/@EventSourceName\">140</Column></Columns>
                </ResultsConfig></ViewerConfig>").expect("Could not create new event viewer custom view for GameMon Event Logs!!");
            }
        };
    
        log!("GameMon Started...", "w");

        let last_error = std::io::Error::last_os_error().to_string();
    
        if last_error.contains("GameMon"){
            log!(format!("Last shutdown reason: CRASH"), "e");
        } else {
            log!(format!("Last shutdown reason: {}", get_value(&hkey, "defaults".to_string(), "exit_reason".to_string())), "w");
        }
    
    }
}