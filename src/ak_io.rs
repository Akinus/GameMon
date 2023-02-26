// Rust Programming Language
// #####################################################################
// File: ak_io.rs                                                      #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:39:37                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Wed, 22 Feb 2023 @ 22:00:58                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
pub mod read {
    use active_win_pos_rs::get_active_window;
    use std::{cmp::Ordering, path::Path};
    use sysinfo::{Pid, ProcessExt, System, SystemExt, PidExt};
    use winapi::um::winuser::{GetLastInputInfo, LASTINPUTINFO, PLASTINPUTINFO};
    use winreg::{
        enums::{RegDisposition, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    use crate::{
        ak_io::write::{reg_section_new, reg_write_value, write_key},
        ak_utils::{
            dark_hours,
            macros::{d_quote, log},
            HKEY,
        },
    };

    pub fn get_pid<T>(pname: T) -> Result<u32, String>
    where
        T: ToString,
    {
        let pname = pname.to_string();
        let mut pids = Vec::new();

        if pname == "" {
            return Err("No PID provided!".to_string());
        };

        let mut s = System::new();
        s.refresh_processes();
        let procs = s.processes_by_exact_name(&pname);

        for process in procs {
            let ox = process.parent().unwrap().to_string();
            if ox == "0" {
                continue;
            } else {
                pids.push(ox.parse::<u32>().unwrap());
            }
        }

        let r = match pids.is_empty() {
            true => Err("Program Not Found!".to_string()),
            false => Ok(pids.last().unwrap().to_owned()),
        };

        return r;
    }

    // pub fn get_cmd_line<T>(pname: T) -> Result<String, String>
    // where
    //     T: ToString,
    // {
    //     let pname = pname.to_string();
    //     let mut cmds = Vec::new();

    //     if pname == "" {
    //         return Err("No PID provided!".to_string());
    //     };

    //     let mut s = System::new();
    //     s.refresh_processes();
    //     let procs = s.processes_by_exact_name(&pname);

    //     for process in procs {
    //         let ox = process.parent().unwrap().to_string();
    //         if ox == "0" {
    //             continue;
    //         } else {
    //             cmds.push(ox);
    //         }
    //     }

    //     let r = match cmds.is_empty() {
    //         true => Err("Program Not Found!".to_string()),
    //         false => Ok(cmds.last().unwrap().to_owned()),
    //     };

    //     return r;
    // }

    pub fn process_exists<T>(pname: T) -> bool where T: ToString {
        
        let r = match get_pid(pname) {
            Ok(_) => true,
            Err(_) => false,
        };

        return r;
    }

    pub fn is_any_process_running<T, U>(exe_check: &Vec<(T, U)>) -> (bool, String)
    where T: ToString, U: ToString
    {

        let mut r = (false, "None".to_string());
        let mut s = sysinfo::System::new();
        s.refresh_processes();
        let x = s.processes().iter().map(|p| p.1.name().to_string()).collect::<Vec<String>>();

        for (exe_name, sec) in exe_check{
            if x.contains(&exe_name.to_string()){
                r = (true, sec.to_string());
            }
        }
        return r
    }

    pub fn window_is_active(process_name: &str) -> bool {
        let active_pid = get_active_window().unwrap().process_id as u32;

        let mut s = System::new();
        s.refresh_process(Pid::from_u32(active_pid));
        let process = s.process(Pid::from_u32(active_pid)).unwrap().name();

        if process == process_name {
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
        pub priority: String,
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
                priority: "".to_string(),
            };
        }
    }

    impl Clone for Instance {
        fn clone(&self) -> Instance {
            Instance {
                exe_name: self.exe_name.clone(),
                game_window_name: self.game_window_name.clone(),
                name_ofahk: self.name_ofahk.clone(),
                path_toahk: self.path_toahk.clone(),
                open_rgbprofile: self.open_rgbprofile.clone(),
                signal_rgbprofile: self.signal_rgbprofile.clone(),
                voice_attack_profile: self.voice_attack_profile.clone(),
                game_or_win: self.game_or_win.clone(),
                running: self.running.clone(),
                running_pid: self.running_pid.clone(),
                other_commands: self.other_commands.clone(),
                priority: self.priority.clone(),
            }
        }
    }

    impl Drop for Instance {
        fn drop(&mut self) {}
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
        pub current_priority: String,
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
                current_priority: "".to_string(),
            };
        }
    }

    pub fn get_defaults() -> Defaults {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        path = Path::new("Software").join("GameMon").join("defaults");
        let sec = hklm.open_subkey(&path).unwrap();
        let mut defaults = Defaults::new();

        for i in gamemon.enum_keys().map(|x| x.unwrap()) {
            match i.as_str() {
                "defaults" => {
                    for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
                        match name.as_str() {
                            "openrgb_path" => defaults.openrgb_path = d_quote!(&value.to_string()),
                            "exit_reason" => defaults.exit_reason = d_quote!(&value.to_string()),
                            "voice_attack_path" => {
                                defaults.voice_attack_path = d_quote!(&value.to_string())
                            }
                            "default_orgb_profile" => {
                                defaults.default_orgb_profile = d_quote!(&value.to_string())
                            }
                            "default_srgb_profile" => {
                                defaults.default_srgb_profile = d_quote!(&value.to_string())
                            }
                            "screensaver_orgb_profile" => {
                                defaults.screensaver_orgb_profile = d_quote!(&value.to_string())
                            }
                            "screensaver_srgb_profile" => {
                                defaults.screensaver_srgb_profile = d_quote!(&value.to_string())
                            }
                            "night_hour_orgb_profile" => {
                                defaults.night_hour_orgb_profile = d_quote!(&value.to_string())
                            }
                            "night_hour_srgb_profile" => {
                                defaults.night_hour_srgb_profile = d_quote!(&value.to_string())
                            }
                            "orgb_port" => defaults.orgb_port = d_quote!(&value.to_string()),
                            "orgb_address" => defaults.orgb_address = d_quote!(&value.to_string()),
                            "gameon" => defaults.gameon = d_quote!(&value.to_string()),
                            "window_flag" => defaults.window_flag = d_quote!(&value.to_string()),
                            "current_priority" => {
                                defaults.current_priority = d_quote!(&value.to_string())
                            }

                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        return defaults;
    }

    pub fn get_section<T>(sec_name: T) -> Instance
    where
        T: ToString,
    {
        let sec_name = sec_name.to_string();
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        path = Path::new("Software").join("GameMon").join(&sec_name);
        let sec = hklm.open_subkey(&path).unwrap();
        let mut section = Instance::new();

        for i in gamemon.enum_keys().map(|x| x.unwrap()) {
            if i == sec_name {
                for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
                    match name.as_str() {
                        "exe_name" => section.exe_name = d_quote!(&value.to_string()),
                        "game_window_name" => {
                            section.game_window_name = d_quote!(&value.to_string())
                        }
                        "name_ofahk" => section.name_ofahk = d_quote!(&value.to_string()),
                        "path_toahk" => section.path_toahk = sec.get_value("path_toahk").unwrap(),
                        "open_rgbprofile" => section.open_rgbprofile = d_quote!(&value.to_string()),
                        "signal_rgbprofile" => {
                            section.signal_rgbprofile = d_quote!(&value.to_string())
                        }
                        "voice_attack_profile" => {
                            section.voice_attack_profile = d_quote!(&value.to_string())
                        }
                        "game_or_win" => section.game_or_win = d_quote!(&value.to_string()),
                        "running" => section.running = d_quote!(&value.to_string()),
                        "running_pid" => section.running_pid = d_quote!(&value.to_string()),
                        "other_commands" => {
                            section.other_commands = sec.get_value("other_commands").unwrap()
                        }
                        "priority" => section.priority = d_quote!(&value.to_string()),
                        _ => (),
                    }
                }
            }
        }

        return section;
    }

    pub fn get_idle() -> Instance {
        let defaults = get_defaults();
        let mut section = get_section("Idle");

        if dark_hours() {
            section.open_rgbprofile = defaults.night_hour_orgb_profile;
            section.signal_rgbprofile = defaults.night_hour_srgb_profile;
            section.game_window_name = "Night".to_owned();
        } else if section.game_or_win == "Yes" {
            section.open_rgbprofile = defaults.screensaver_orgb_profile;
            section.signal_rgbprofile = defaults.screensaver_srgb_profile;
            section.game_window_name = "Day".to_owned();
        } else {
            section.game_window_name = "Day".to_owned();
        }

        return section;
    }

    pub fn filtered_keys<T>(
        enum_keys: &mut Vec<(String, Instance)>,
        current_priority: T
    ) -> Vec<(String, Instance)>
    where
        T: ToString
    {
        let current_priority = current_priority.to_string().parse::<u32>().unwrap();
        enum_keys
            .iter_mut()
            .filter_map(|(name, instance)| {
                if name == "Idle" || name == "General" || name == "defaults" {
                    None
                } else if process_exists(&instance.exe_name)
                    && ((instance.game_or_win == "Game"
                        && instance.priority.parse::<u32>().unwrap() >= current_priority)
                        || (window_is_active(&instance.exe_name)
                            && get_value(
                                HKEY,
                                gamemon_value(HKEY, "current_profile").to_owned(),
                                "game_or_win",
                            ) != "Game"))
                {
                    Some((name.clone(), instance.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn ss_get(key_name: &'static str) -> String {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
        let screen_s = desktop.get_value(&key_name).unwrap();

        return screen_s;
    }

    pub fn get_value<T, U>(hkey: &RegKey, section: T, key: U) -> String
    where
        T: ToString,
        U: ToString,
    {
        let section = section.to_string();
        let key = key.to_string();
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(section);
        let gamemon = hklm.open_subkey(&path).unwrap();
        let return_value = gamemon.get_value(key).unwrap();
        return_value
    }

    pub fn gamemon_value(hkey: &RegKey, key: &str) -> String {
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon");
        let gamemon = hklm.open_subkey(&path).unwrap();
        gamemon.get_value(key).unwrap()
    }

    pub fn reg_check(hkey: &RegKey) {
        let hklm = hkey;
        let mut path = Path::new("Software").join("GameMon");
        let disp = hklm.create_subkey(&path).unwrap().1;

        match disp {
            RegDisposition::REG_CREATED_NEW_KEY => {
                log!(format!("A new key has been created at {:?}", &path));
                let ini_file: String = std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                match reg_write_value(
                    &hkey,
                    &path,
                    "InstallDir".to_string(),
                    format!("{}", &ini_file),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value {} to {:?}\\InstallDir",
                            &ini_file, &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value {} to {:?}\\InstallDir",
                                &ini_file, &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(&hkey, &path, "display".to_string(), (&"on").to_string()) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"on\" to {:?}\\display",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"on\" to {:?}\\display",
                                &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(&hkey, &path, "exit_reason".to_string(), (&"default").to_string()) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"default\" to {:?}\\exit_reason",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"default\" to {:?}\\exit_reason",
                                &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(
                    &hkey,
                    &path,
                    "current_profile".to_string(),
                    (&"General").to_string(),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"General\" to {:?}\\current_profile",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"General\" to {:?}\\current_profile",
                                &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(
                    &hkey,
                    &path,
                    "current_priority".to_string(),
                    (&"0").to_string(),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"0\" to {:?}\\current_priority",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"0\" to {:?}\\current_priority",
                                &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(
                    &hkey,
                    &path,
                    "last_profile".to_string(),
                    (&"General").to_string(),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"General\" to {:?}\\last_profile",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"General\" to {:?}\\last_profile",
                                &path
                            ),
                            "e"
                        );
                    }
                };
                match reg_write_value(
                    &hkey,
                    &path,
                    "current_profile_activated".to_string(),
                    (&"true").to_string(),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"General\" to {:?}\\current_profile_activated",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(&format!("Could not write value \"General\" to {:?}\\current_profile_activated", &path), "e");
                    }
                };
                match reg_write_value(&hkey, &path, "night".to_string(), (&"false").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value \"false\" to {:?}\\night", &path));
                    }
                    Err(_) => {
                        log!(
                            &format!("Could not write value \"false\" to {:?}\\night", &path),
                            "e"
                        );
                    }
                };
                match reg_write_value(&hkey, &path, "idle".to_string(), (&"false").to_string()) {
                    Ok(_) => {
                        log!(&format!("Wrote value \"false\" to {:?}\\idle", &path));
                    }
                    Err(_) => {
                        log!(
                            &format!("Could not write value \"false\" to {:?}\\idle", &path),
                            "e"
                        );
                    }
                };
                match reg_write_value(
                    &hkey,
                    &path,
                    "last_other_commands".to_string(),
                    (&"General").to_string(),
                ) {
                    Ok(_) => {
                        log!(&format!(
                            "Wrote value \"General\" to {:?}\\last_other_commands",
                            &path
                        ));
                    }
                    Err(_) => {
                        log!(
                            &format!(
                                "Could not write value \"General\" to {:?}\\last_other_commands",
                                &path
                            ),
                            "e"
                        );
                    }
                };

                for i in ["General", "Idle", "defaults"] {
                    path = Path::new("Software").join("GameMon").join(&i);
                    match &i {
                        &"defaults" => {
                            let disp = hklm.create_subkey(&path).unwrap().1;

                            match disp {
                                RegDisposition::REG_CREATED_NEW_KEY => {
                                    log!(format!("A new section has been created at {:?}", &path));

                                    for i in [
                                        "openrgb_path".to_string(),
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
                                        "window_flag".to_string(),
                                    ] {
                                        match reg_write_value(
                                            &hkey,
                                            &path,
                                            String::from(&i),
                                            "".to_string(),
                                        ) {
                                            Ok(_) => {
                                                log!(&format!("Created empty value {}", &i));
                                            }
                                            Err(_) => {
                                                log!(
                                                    &format!(
                                                        "Could not write value {} to {:?}",
                                                        &i, &path
                                                    ),
                                                    "e"
                                                );
                                            }
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
                                    write_key(
                                        &hkey,
                                        &section_name,
                                        "screensaver_orgb_profile",
                                        "General",
                                    );
                                    write_key(
                                        &hkey,
                                        &section_name,
                                        "screensaver_srgb_profile",
                                        "Screen Ambience",
                                    );
                                }
                                RegDisposition::REG_OPENED_EXISTING_KEY => {
                                    log!(&"An existing key has been opened".to_string());
                                }
                            }
                        }
                        o => reg_section_new(&hkey, o.to_string()),
                    }
                }

                let mut section_name = "General".to_string();
                write_key(&hkey, &section_name, "OpenRGBprofile", "General");
                write_key(&hkey, &section_name, "SignalRGBprofile", "General");
                write_key(&hkey, &section_name, "game_or_win", "Game");
                write_key(&hkey, &section_name, "priority", "0");

                section_name = "Idle".to_string();
                write_key(&hkey, &section_name, "exeName", "300");
                write_key(&hkey, &section_name, "gameWindowName", "2100-0600");
                write_key(&hkey, &section_name, "game_or_win", "Game");
                write_key(&hkey, &section_name, "priority", "4");
            }
            RegDisposition::REG_OPENED_EXISTING_KEY => {
                log!(&"An existing key has been opened".to_string());
            }
        }
    }

    pub fn user_idle() -> bool {
        let wait_time = get_value(HKEY, "Idle", "exe_name").parse::<u64>().unwrap();

        let now = unsafe { winapi::um::sysinfoapi::GetTickCount() };
        let mut last_input_info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        let p_last_input_info: PLASTINPUTINFO = &mut last_input_info as *mut LASTINPUTINFO;

        let ok = unsafe { GetLastInputInfo(p_last_input_info) } != 0;

        let idle_seconds = match ok {
            true => {
                let millis = now - last_input_info.dwTime;
                Ok(std::time::Duration::from_millis(millis as u64))
            }
            false => Err("GetLastInputInfo failed".to_string()),
        }
        .unwrap()
        .as_secs();

        if idle_seconds.cmp(&(&wait_time)) == Ordering::Greater {
            let _v = reg_write_value(
                &RegKey::predef(HKEY_LOCAL_MACHINE),
                &Path::new("Software").join("GameMon"),
                "idle".to_string(),
                "true".to_string(),
            );
            return true;
        } else {
            let _v = reg_write_value(
                &RegKey::predef(HKEY_LOCAL_MACHINE),
                &Path::new("Software").join("GameMon"),
                "idle".to_string(),
                "false".to_string(),
            );
            
            return false;
        }
    }
}

pub mod write {
    use crate::{
        ak_io::read::{get_value, user_idle},
        ak_utils::{dark_hours, macros::log},
    };
    use std::path::{Path, PathBuf};

    use winreg::{
        enums::{
            RegDisposition::{REG_CREATED_NEW_KEY, REG_OPENED_EXISTING_KEY},
            HKEY_LOCAL_MACHINE,
        },
        RegKey,
    };

    pub fn write_key(hkey: &RegKey, sec_name: &String, key_name: &'static str, key_value: &str) {
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(&sec_name);
        let key = hklm.create_subkey(&path).unwrap().0;

        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn delete_section(hkey: &RegKey, sec_name: &String) {
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon").join(sec_name);
        hklm.delete_subkey_all(path).unwrap();
    }

    pub fn ss_set(hkey: &RegKey, key_name: &'static str, key_value: &str) {
        let hkcu = hkey;
        let path = Path::new("Control Panel").join("Desktop");
        let key = hkcu.create_subkey(&path).unwrap().0;

        return key.set_value(&key_name, &key_value).unwrap();
    }

    pub fn reset_running(hkey: &RegKey) {
        let hklm = hkey;
        let path = Path::new("Software").join("GameMon");
        let game_mon = hklm.open_subkey(&path).unwrap();

        for sec in game_mon.enum_keys().map(|x| x.unwrap()) {
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
        // let _ = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
        //     , "current_profile".to_string()
        //     , "General".to_string());
        // let _ = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
        //     , "current_priority".to_string()
        //     , get_value(HKEY, "General", "priority"));
        // let _ = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
        //     , "current_profile_activated".to_string()
        //     , "General".to_string());
        // let _ = reg_write_value(&RegKey::predef(HKEY_LOCAL_MACHINE), &Path::new("Software").join("GameMon")
        //     , "last_profile".to_string()
        //     , "General".to_string());

        let _u = user_idle();

        let _n = dark_hours();

        log!("Running values reset.".to_string(), "w");
    }

    pub fn write_section(hkey: &RegKey, sec_name: &String) {
        write_key(&hkey, sec_name, "exe_name", "");
        write_key(&hkey, sec_name, "game_window_name", "");
        write_key(&hkey, sec_name, "name_ofahk", "");
        write_key(&hkey, sec_name, "path_toahk", "");
        write_key(&hkey, sec_name, "open_rgbprofile", "");
        write_key(&hkey, sec_name, "voice_attack_profile", "");
        write_key(&hkey, sec_name, "signal_rgbprofile", "");
        write_key(&hkey, sec_name, "game_or_win", "");
        write_key(&hkey, sec_name, "priority", "");
        write_key(&hkey, sec_name, "running", "");
        write_key(&hkey, sec_name, "running_pid", "");
        write_key(&hkey, sec_name, "other_commands", "");
    }

    pub fn reg_write_value<T, U>(
        hkey: &RegKey,
        path: &PathBuf,
        name: T,
        value: U,
    ) -> Result<(), std::io::Error> where T: ToString, U: ToString {
        let name = name.to_string();
        let value = value.to_string();
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
                for i in [
                    "exe_name".to_string(),
                    "game_window_name".to_string(),
                    "name_ofahk".to_string(),
                    "path_toahk".to_string(),
                    "open_rgbprofile".to_string(),
                    "signal_rgbprofile".to_string(),
                    "voice_attack_profile".to_string(),
                    "game_or_win".to_string(),
                    "running".to_string(),
                    "running_pid".to_string(),
                    "other_commands".to_string(),
                ] {
                    match reg_write_value(&hkey, &path, String::from(&i), "".to_string()) {
                        Ok(_) => {
                            log!(&format!("Created empty value {}", &i));
                        }
                        Err(_) => {
                            log!(&format!("Could not write value {} to {:?}", &i, &path), "e");
                        }
                    };
                }
            }
            REG_OPENED_EXISTING_KEY => (),
        }
    }
}

pub mod logging {
    use std::path::Path;

    use crate::{ak_io::read::get_value, ak_utils::{macros::log, HKEY}};
    use winreg::RegKey;

    use super::{read::gamemon_value, write::reg_write_value};

    pub fn initialize_log(hkey: &RegKey) {
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

        let mut custom_view =
            "C:\\ProgramData\\Microsoft\\Event Viewer\\Views\\gamemon_trace_logs.xml";
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

        let last_error = gamemon_value(HKEY, "exit_reason");

        if last_error == "default".to_owned() {
            log!(
                format!("GameMon Started...\nLast shutdown reason: CRASH"),
                "e"
            );
        } else {
            log!(
                format!(
                    "GameMon Started...\nLast shutdown reason: {}",
                    get_value(&hkey, "defaults".to_string(), "exit_reason".to_string())
                ),
                "w"
            );
        }

        let path = Path::new("Software").join("GameMon");
        reg_write_value(HKEY, &path, "exit_reason", "default").unwrap();

    }
}
