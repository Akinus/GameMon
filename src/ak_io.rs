// Rust Programming Language
// #####################################################################
// File: ak_io.rs                                                      #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:39:37                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Fri, 16 Dec 2022 @ 20:53:54                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
pub mod read {
    use std::path::Path;
    use sysinfo::{System, SystemExt, Pid, ProcessExt};
    use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, RegDisposition}};

    use crate::{ak_utils::macros::
    {
        d_quote,
        log
    }, ak_io::write::{write_key, reg_section_new, reg_write_value}};

    

    pub fn name_by_pid(pid: Pid) -> Result<String, String>{
        let s = System::new_all();
        if let Some(process) = s.process(pid){
            return Ok(process.name().to_owned());
        }

        return Err("None".to_string());
    }

    pub fn get_pid(pname: Option<&str>) -> Result<u32, &str>{
        match pname {
            Some(i) => {
                
                let s = System::new_all();
                let procs = s.processes_by_exact_name(i);
                
                match Some(procs) {
                    Some(p) => {
                        
                        for process in p {
                            
                            let ox = process.parent().unwrap().to_string();
                            return Ok(ox.parse::<u32>().unwrap());
                        };
     
                    },
                    None => {
                        return Err(&"No Match Found")
                    }
                };
                
            },
            None => return Err(&"No Match Found")
        }
        return Ok(0)
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
                        "other_commands" => section.other_commands = d_quote!(&value.to_string()),
                        "priority" => section.priority = d_quote!(&value.to_string()),
                        _ => ()
                    }
                }
            }
           
        }
    
        return section
    }

    pub fn ss_get(key_name: &'static str) -> String{
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
        let screen_s: String = desktop.get_value(&key_name).unwrap();
    
        return screen_s;
    }
    
    pub fn get_value(section: String, key: String) -> String{
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon").join(section);
        let gamemon = hklm.open_subkey(&path).unwrap();
        gamemon.get_value(key).unwrap()
    }

    
    pub fn reg_check(){
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut path = Path::new("Software").join("GameMon");
        let disp = hklm.create_subkey(&path).unwrap().1;
    
        match disp {
            RegDisposition::REG_CREATED_NEW_KEY => {
                log!(format!("A new key has been created at {:?}", &path));
                let ini_file: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
                match reg_write_value(&path, "InstallDir".to_string(), format!("{}", &ini_file)) {
                    Ok(_) => {
                        log!(&format!("Wrote value {:?} to {}\\InstallDir", &path, &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value {:?} to {}\\InstallDir", &path, &ini_file), "e");
                    },
                };
                match reg_write_value(&path, "display".to_string(), (&"on").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value {:?} to {}\\display", &path, &ini_file));
                    },
                    Err(_) => {
                        log!(&format!("Could not write value {:?} to {}\\display", &path, &ini_file), "e");
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
                                        match reg_write_value(&path, String::from(&i), "".to_string()) {
                                            Ok(_) => {
                                                log!(&format!("Created empty value {}", &i));
                                            },
                                            Err(_) => {
                                                log!(&format!("Could not write value {} to {:?}", &i, &path), "e");
                                            },
                                        };
                                    }
                                    let section_name = "defaults".to_string();
                                    write_key(&section_name, "exit_reason", "");
                                    write_key(&section_name, "pathToSchemas", "");
                                    write_key(&section_name, "orgb_port", "6742");
                                    write_key(&section_name, "orgb_address", "127.0.0.1");
                                    write_key(&section_name, "gameon", "False");
                                    write_key(&section_name, "current_priority", "0");
                                    write_key(&section_name, "running", "");
                                    write_key(&section_name, "window_flag", "General");
                                    write_key(&section_name, "screensaver_orgb_profile", "General");
                                    write_key(&section_name, "screensaver_srgb_profile", "Screen Ambience");
                                },
                                RegDisposition::REG_OPENED_EXISTING_KEY => {
                                    log!(&"An existing key has been opened".to_string());
                                },
                            }
                        },
                        o => {
                            reg_section_new(o.to_string())
                        }
                    }
                    
                }
            
                let mut section_name = "General".to_string();
                write_key(&section_name, "OpenRGBprofile", "General");
                write_key(&section_name, "SignalRGBprofile", "General");
                write_key(&section_name, "game-or-win", "Game");
                write_key(&section_name, "priority", "0");
            
                section_name = "Idle".to_string();
                write_key(&section_name, "exeName", "300");
                write_key(&section_name, "gameWindowName", "2100-0600");
                write_key(&section_name, "game-or-win", "Game");
                write_key(&section_name, "priority", "4");
            },
            RegDisposition::REG_OPENED_EXISTING_KEY => {
                log!(&"An existing key has been opened".to_string());
            },
        }
    
        
    
        
    }

}

pub mod write {
    use std::path::{Path, PathBuf};
    use crate::ak_utils::macros::{
        log
    };

    use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, RegDisposition::{REG_CREATED_NEW_KEY, REG_OPENED_EXISTING_KEY}}};


    pub fn write_key(sec_name: &String, key_name: &'static str, key_value: &str){
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon").join(&sec_name);
        let key = hklm.create_subkey(&path).unwrap().0;
    
        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn delete_section(sec_name: &String){
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon").join(sec_name);
        hklm.delete_subkey_all(path).unwrap();
    }

    pub fn ss_set(key_name: &'static str, key_value: &str){
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Control Panel").join("Desktop");
        let key = hkcu.create_subkey(&path).unwrap().0;
    
        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn reset_running() -> String{
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon");
        let game_mon = hklm.open_subkey(&path).unwrap();
        
        for sec in game_mon.enum_keys().map(|x| x.unwrap()){
            match &sec.as_str() {
                &"General" => (),
                &"defaults" => (),
                _ => {
                    write_key(&sec, "running", "False");
                }
            }
        }
    
        write_key(&"defaults".to_string(), "gameon", "False");
        write_key(&"General".to_string(), "running", "True");
        write_key(&"General".to_string(), "running_pid", "0");
        return "Running values reset.".to_string();
    }

    pub fn write_section(sec_name: &String){
        write_key(sec_name, "exeName", "");
        write_key(sec_name, "gameWindowName", "");
        write_key(sec_name, "nameOfahk", "");
        write_key(sec_name, "pathToahk", "");
        write_key(sec_name, "OpenRGBprofile", "");
        write_key(sec_name, "voiceAttackProfile", "");
        write_key(sec_name, "SignalRGBprofile", "");
        write_key(sec_name, "game-or-win", "");
        write_key(sec_name, "priority", "");
        write_key(sec_name, "running", "");
        write_key(sec_name, "running_pid", "");
    }

    pub fn reg_write_value(path: &PathBuf, name: String, value: String) -> Result<(), std::io::Error> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm.create_subkey(&path).unwrap().0;
    
        return key.set_value(&name, &value);
    }

    pub fn reg_section_new(sec: String) {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
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
                    match reg_write_value(&path, String::from(&i), "".to_string()) {
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
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    use crate::ak_utils::macros::{
        log
    };

    
    pub fn initialize_log(){
        eventlog::register("GameMon Log").unwrap();
        eventlog::init("GameMon Log", log::Level::Trace).unwrap();
        
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
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
    
    }
}