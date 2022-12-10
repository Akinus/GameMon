// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 05 Nov 2022 @ 14:53:07                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################
#![allow(non_snake_case)]
#![cfg_attr(
    all(
      target_os = "windows",
    //   not(feature = "console"),
    ),
    windows_subsystem = "windows",
  )]

//   Import Data ####
// extern crate winreg;
use sysinfo::{ProcessExt, System, SystemExt, Pid};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use active_win_pos_rs::get_active_window;
use std::{process::{Command}, os::windows::{process::CommandExt}, io::Write, fs::{OpenOptions, File}, path::{Path, PathBuf}, cmp::Ordering};
use chrono::{Local, NaiveTime};
use reqwest::{self, header};
use {std::sync::mpsc, tray_item::TrayItem};
use winreg::{enums::*};
use winreg::RegKey;
use user_idle::UserIdle;
use windows_win::{raw::window::{
    get_by_title,
    get_thread_process_id
}};
use winsafe::{prelude::*, WString};
use winsafe::{gui, POINT, SIZE, co::{COLOR, WS, SS}};

// Environment Variables
const CREATE_NO_WINDOW: u32 = 0x08000000;

// Macros

macro_rules! exit_app {
    
    ($a:expr) => {
        {
            log!("Exiting.  Reason: Shutdown", "w");
            log!(format!("{}", reset_running()), "w");
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log!(format!("All ahk scripts are closed"), "w");

            write_key("defaults", "exit_reason", "Shutdown");
            std::process::exit($a);
        }
    };

    ($b:expr) => {
        {
            log!(format!("Exiting. Reason: {}", $b), "w");
            log!(format!("{}", reset_running()), "w");
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log!(format!("All ahk scripts are closed"), "w");

            write_key("defaults", "exit_reason", $b);
            std::process::abort();           
        }
    };

    ($a:expr,$b:expr) => {
        {
            log!(format!("Exiting. Reason: {}", $b), "w");
            log!(format!("{}", reset_running()), "w");
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log!(format!("All ahk scripts are closed"), "w");

            write_key("defaults", "exit_reason", $b);
            std::process::exit($a);
        }
    };
    
    () => {
        {
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            std::process::abort();
        }
    };
}

macro_rules! log {
    ($a:expr) => {
        {
            let now = timestamp();
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
            let mut log_file: String = g_key.get_value("InstallDir").unwrap();
            log_file.push_str("\\gamemon.log");

            let data = format!("{}: INFO: {}", &now, $a);

            let mut lfile = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&log_file)
                .unwrap();
            write!(lfile, "{}", format!("{data}\n")).unwrap();
        
        }
    };

    ($a:expr,$b:expr) => {
        {
            let now = timestamp();
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
            let mut log_file: String = g_key.get_value("InstallDir").unwrap();
            log_file.push_str("\\gamemon.log");

            match $b {
                "i" => {
                    let data = format!("{}: INFO: {}", &now, $a);
                    let mut lfile = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&log_file)
                        .unwrap();
                    write!(lfile, "{}", format!("{data}\n")).unwrap();
                },
                "d" => {
                    let data = format!("{}: DEBUG: {}", &now, $a);
                    let mut lfile = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&log_file)
                        .unwrap();
                    write!(lfile, "{}", format!("{data}\n")).unwrap();                    
                },
                "e" => {
                    let data = format!("{}: ERROR: {}", &now, $a);
                    let mut lfile = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&log_file)
                        .unwrap();
                    write!(lfile, "{}", format!("{data}\n")).unwrap();                    
                },
                "w" => {
                    let data = format!("{}: WARNING: {}", &now, $a);
                    let mut lfile = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&log_file)
                        .unwrap();
                    write!(lfile, "{}", format!("{data}\n")).unwrap();                    
                },
                _ => (),
            }
                   
        }
    };

    () => {
        {
            let now = timestamp();
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
            let mut log_file: String = g_key.get_value("InstallDir").unwrap();
            log_file.push_str("\\gamemon.log");

            let data = format!("{}: DEBUG: BREAK BREAK BREAK ----------------", &now);

            let mut lfile = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&log_file)
                .unwrap();
            write!(lfile, "{}", format!("{data}\n")).unwrap();           
        }
    }
}

// Enums, Structs, Impl

#[derive(Debug)]
enum Message {
    Quit,
    Gui,
    Defaults,
}

struct Instance {
    exe_name: String,
    game_window_name: String,
    name_ofahk: String,
    path_toahk: String,
    open_rgbprofile: String,
    signal_rgbprofile: String,
    voice_attack_profile: String,
    game_or_win: String,
    running: String,
    running_pid: String,
    other_commands: String
}

impl Instance {
    fn new() -> Instance {
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
        }
    }
}

struct Defaults {
    openrgb_path: String,
    exit_reason: String,
    voice_attack_path: String,
    default_orgb_profile: String,
    default_srgb_profile: String,
    screensaver_orgb_profile: String,
    screensaver_srgb_profile: String,
    night_hour_orgb_profile: String,
    night_hour_srgb_profile: String,
    orgb_port: String,
    orgb_address: String,
    gameon: String,
    window_flag: String
}

impl Defaults {
    fn new() -> Defaults {
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
        }
    }
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        exit_app!();
    }
}

// Run Extra Commands

// GUI
#[derive(Clone)]
pub struct DefaultsWindow {
    wnd: gui::WindowMain,
    label_openRGBpath: gui::Label,
    edit_openRGBpath: gui::Edit,
    label_voiceAttackPath: gui::Label,
    edit_voiceAttackPath: gui::Edit,
    label_defaultORGBProfile: gui::Label,
    edit_defaultORGBProfile: gui::Edit,
    label_defaultSRGBProfile: gui::Label,
    edit_defaultSRGBProfile: gui::Edit,
    label_screensaver_orgb_profile: gui::Label,
    edit_screensaver_orgb_profile: gui::Edit,
    label_screensaver_srgb_profile: gui::Label,
    edit_screensaver_srgb_profile: gui::Edit,
    label_night_hour_orgb_profile: gui::Label,
    edit_night_hour_orgb_profile: gui::Edit,
    label_night_hour_srgb_profile: gui::Label,
    edit_night_hour_srgb_profile: gui::Edit,
    label_orgb_port: gui::Label,
    edit_orgb_port: gui::Edit,
    label_orgb_address: gui::Label,
    edit_orgb_address: gui::Edit,
    btn_save: gui::Button,
    btn_close: gui::Button,
}

impl DefaultsWindow {
    pub fn new() -> Self {
        let last_y = 10;

        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "GameMon - Default Settings".to_owned(),
                class_icon: gui::Icon::Str(WString::from_str("my-icon-name")),
                size: SIZE::new(520, 520),
                ..Default::default() 
            },
        );
        //------------------------------------------------------
        let label_openRGBpath = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB Path to executable".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_openRGBpath = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_voiceAttackPath = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "VoiceAttack Path to executable".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_voiceAttackPath = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_defaultORGBProfile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default OpenRGB Profile".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_defaultORGBProfile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_defaultSRGBProfile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default SignalRGB Profile".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_defaultSRGBProfile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_screensaver_orgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default OpenRGB Profile for Screensaver".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_screensaver_orgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_screensaver_srgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default SignalRGB Profile for Screensaver".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_screensaver_srgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_night_hour_orgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default OpenRGB Profile for Night Hours".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_night_hour_orgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_night_hour_srgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Default SignalRGB Profile for Night Hours".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_night_hour_srgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let label_orgb_address = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB Address".to_owned(),
                size: SIZE::new(100,20),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let edit_orgb_address = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 100,
                position: POINT::new(120, last_y),
                ..Default::default()
            },
        );

        //------------------------------------------------------

        let label_orgb_port = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB port".to_owned(),
                size: SIZE::new(100,20),
                position: POINT::new(230, last_y),
                ..Default::default()
            },
        );

        let edit_orgb_port = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 100,
                position: POINT::new(330, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;
        //------------------------------------------------------

        let btn_save = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Save".to_owned(),
                width: 150,
                position: POINT::new(200, last_y),
                ..Default::default()
            },
        );

        let btn_close = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Close".to_owned(),
                width: 150,
                position: POINT::new(360, last_y),
                ..Default::default()
            },
        );

        let new_self = Self { wnd,
            label_openRGBpath,
            edit_openRGBpath,
            label_voiceAttackPath,
            edit_voiceAttackPath,
            label_defaultORGBProfile,
            edit_defaultORGBProfile,
            label_defaultSRGBProfile,
            edit_defaultSRGBProfile,
            label_screensaver_orgb_profile,
            edit_screensaver_orgb_profile,
            label_screensaver_srgb_profile,
            edit_screensaver_srgb_profile,
            label_night_hour_orgb_profile,
            edit_night_hour_orgb_profile,
            label_night_hour_srgb_profile,
            edit_night_hour_srgb_profile,
            label_orgb_port,
            edit_orgb_port,
            label_orgb_address,
            edit_orgb_address,
            btn_save,
            btn_close,
        };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        self.wnd.on().wm_create({ // happens once, right after the window is created
			let self2 = self.clone();
			move |_| {
                let defaults = get_defaults();

                self2.edit_openRGBpath.set_text(&defaults.openrgb_path);
                self2.edit_voiceAttackPath.set_text(&defaults.voice_attack_path);
                self2.edit_defaultORGBProfile.set_text(&defaults.default_orgb_profile);
                self2.edit_defaultSRGBProfile.set_text(&defaults.default_srgb_profile);
                self2.edit_screensaver_orgb_profile.set_text(&defaults.screensaver_orgb_profile);
                self2.edit_screensaver_srgb_profile.set_text(&defaults.screensaver_srgb_profile);
                self2.edit_night_hour_orgb_profile.set_text(&defaults.night_hour_orgb_profile);
                self2.edit_night_hour_srgb_profile.set_text(&defaults.night_hour_srgb_profile);
                self2.edit_orgb_port.set_text(&defaults.orgb_port);
                self2.edit_orgb_address.set_text(&defaults.orgb_address);
              
                    
				Ok(0)
			}
		});

        self.btn_save.on().bn_clicked({
            let self2 = self.clone();
            move || {
                write_key(&"defaults", "openRGBPath", self2.edit_openRGBpath.text().as_str());
                write_key(&"defaults", "voiceAttackPath", self2.edit_voiceAttackPath.text().as_str());
                write_key(&"defaults", "defaultORGBProfile", self2.edit_defaultORGBProfile.text().as_str());
                write_key(&"defaults", "defaultSRGBProfile", self2.edit_defaultSRGBProfile.text().as_str());
                write_key(&"defaults", "screensaver_orgb_profile", self2.edit_screensaver_orgb_profile.text().as_str());
                write_key(&"defaults", "screensaver_srgb_profile", self2.edit_screensaver_srgb_profile.text().as_str());
                write_key(&"defaults", "night_hour_orgb_profile", self2.edit_night_hour_orgb_profile.text().as_str());
                write_key(&"defaults", "night_hour_srgb_profile", self2.edit_night_hour_srgb_profile.text().as_str());
                write_key(&"defaults", "orgb_port", self2.edit_orgb_port.text().as_str());
                write_key(&"defaults", "orgb_address", self2.edit_orgb_address.text().as_str());
                msg_box("GameMon", "Saved!");
                Ok(())
            }
        });

        self.btn_close.on().bn_clicked({
            let self2 = self.clone();
            move || {
                self2.wnd.hwnd().DestroyWindow()?;
                Ok(())
            }
        });
    }
}
        
#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,
    btn_add: gui::Button,
    edit_add: gui::Edit,
    btn_delete: gui::Button,
    main_list: gui::ListBox,
    label_exe: gui::Label,
    edit_exe: gui::Edit,
    label_win_name: gui::Label,
    edit_win_name: gui::Edit,
    label_ahk_name: gui::Label,
    edit_ahk_name: gui::Edit,
    label_ahk_path: gui::Label,
    edit_ahk_path: gui::Edit,
    label_orgb_profile: gui::Label,
    edit_orgb_profile: gui::Edit,
    btn_find: gui::Button,
    label_orgb_port: gui::Label,
    edit_orgb_port: gui::Edit,
    label_orgb_address: gui::Label,
    edit_orgb_address: gui::Edit,
    label_srgb_profile: gui::Label,
    edit_srgb_profile: gui::Edit,
    label_va_profile: gui::Label,
    edit_va_profile: gui::Edit,
    label_game_win: gui::Label,
    btn_save: gui::Button,
    radio_game_win: gui::RadioGroup,
    btn_close: gui::Button,
    label_other_commands: gui::Label,
    btn_cmd_add: gui::Button,
    edit_cmd_add: gui::Edit,
    btn_cmd_delete: gui::Button,
    cmd_list: gui::ListBox,

}

impl MyWindow {
    pub fn new() -> Self {
        
        let last_y = 10;

        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "GameMon - Monitor Settings".to_owned(),
                class_icon: gui::Icon::Str(WString::from_str("my-icon-name")),
                class_bg_brush: gui::Brush::Color(COLOR::BTNFACE),
                style: WS::MINIMIZEBOX | 
                    WS::MAXIMIZEBOX | 
                    WS::CAPTION | 
                    WS::SYSMENU | 
                    WS::CLIPCHILDREN | 
                    WS::BORDER | 
                    WS::VISIBLE,
                size: SIZE::new(900, 825),
                ..Default::default() 
            },
        );
        
        let btn_add = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Add".to_owned(),
                width: 150,
                position: POINT::new(20, 10),
                ..Default::default()
            },
        );

        let edit_add = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 150,
                position: POINT::new(20, 40),
                ..Default::default()
            },
        );

        let btn_delete = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Delete".to_owned(),
                width: 150,
                position: POINT::new(20, 70),
                ..Default::default()
            },
        );

        let main_list = gui::ListBox::new(
            &wnd,
            gui::ListBoxOpts{
                size: SIZE::new(200, 485),
                position: POINT::new(180, 10),
                ..Default::default()
            }
        );

        let label_exe = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Process Name".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, 10),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_exe = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 30;

        let label_win_name = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Window Name / Title".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_win_name = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );

        let last_y = last_y + 30;

        let label_ahk_name = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Name of AHK Script to run".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_ahk_name = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );
        
        let last_y = last_y + 30;

        let label_ahk_path = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Path to AHK Script to run".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_ahk_path = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );
        
        let last_y = last_y + 30;

        let label_orgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB Profile to Apply (***REQUIRES OpenRGB Webhooks Plugin***)".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let btn_find = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Find".to_owned(),
                position: POINT::new(800, last_y - 5),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_orgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );

        let last_y = last_y + 30;

        let label_orgb_port = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB Port".to_owned(),
                size: SIZE::new(95,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let edit_orgb_port = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 100,
                position: POINT::new(485, last_y - 2),  
                ..Default::default()
            },
        );

        let label_orgb_address = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "OpenRGB Address".to_owned(),
                size: SIZE::new(100,20),
                position: POINT::new(590, last_y),
                ..Default::default()
            },
        );

        let edit_orgb_address = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 200,
                position: POINT::new(690, last_y - 2),  
                ..Default::default()
            },
        );

        let last_y = last_y + 30;

        let label_srgb_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "SignalRGB Profile to Apply".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_srgb_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );
        
        let last_y = last_y + 30;

        let label_va_profile = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "VoiceAttack Profile to Apply".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 25;

        let edit_va_profile = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 500,
                position: POINT::new(390, last_y),  
                ..Default::default()
            },
        );
        
        let last_y = last_y + 30;

        let label_game_win = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Apply settings on....".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(390, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 28;

        let radio_game_win = gui::RadioGroup::new(
            &wnd, &[
                gui::RadioButtonOpts {
                    text: "Game".to_owned(),
                    selected: false,
                    position: POINT::new(390, last_y + 2),  
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "Window".to_owned(),
                    selected: false,
                    position: POINT::new(460, last_y + 2),  
                    ..Default::default()
                },

            ]
            
        );

        let last_y = 500;

        let label_other_commands = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                label_style: SS::CENTER | SS::NOTIFY,
                text: "------------------------------------------------------------- \
                Other Commands -------------------------------------------------------------".to_owned(),
                size: SIZE::new(900,20),
                position: POINT::new(0, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 28;

        let btn_cmd_add = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                width: 150,
                text: "&Add".to_owned(),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let edit_cmd_add = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                width: 710,
                position: POINT::new(180, last_y),  
                ..Default::default()
            },
        );

        let last_y = last_y + 28;

        let btn_cmd_delete = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                width: 150,
                text: "&Delete".to_owned(),
                position: POINT::new(10, last_y),
                ..Default::default()
            },
        );

        let cmd_list = gui::ListBox::new(
            &wnd,
            gui::ListBoxOpts{
                size: SIZE::new(710, 200),
                position: POINT::new(180, last_y),
                ..Default::default()
            }
        );

        let last_y = last_y + 205;

        let btn_save = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Save".to_owned(),
                position: POINT::new(800, last_y),
                ..Default::default()
            },
        );

        let last_y = last_y + 28;

        let btn_close = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Close".to_owned(),
                position: POINT::new(800, last_y),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_add,
            btn_delete, main_list,
            edit_exe, label_exe,
            label_win_name,
            edit_win_name,
            label_ahk_name,
            edit_ahk_name,
            label_ahk_path,
            edit_ahk_path,
            label_orgb_profile,
            btn_find,
            edit_orgb_profile,
            label_orgb_port,
            edit_orgb_port,
            label_orgb_address,
            edit_orgb_address,
            label_srgb_profile,
            edit_srgb_profile,
            label_va_profile,
            edit_va_profile,
            label_game_win,
            btn_save,
            radio_game_win,
            btn_close,
            edit_add,
            label_other_commands,
            btn_cmd_add,
            edit_cmd_add,
            btn_cmd_delete,
            cmd_list,
        };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        self.wnd.on().wm_create({ // happens once, right after the window is created
			let self2 = self.clone();
			move |_| {
                let mut item_vec = Vec::new();
                let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                let path = Path::new("Software").join("GameMon");
                let game_mon = hklm.open_subkey(&path).unwrap();
                
                for sec in game_mon.enum_keys().map(|x| x.unwrap()){
                    match &sec.as_str() {
                        &"defaults" => (),
                        _ => item_vec.push(sec)
                    }
                };

                item_vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                for i in item_vec{
                    self2.main_list.items().add(&[i]);
                }

                

                
                let defaults = get_defaults();
                self2.edit_orgb_port.set_text(&defaults.orgb_port);
                self2.edit_orgb_address.set_text(&defaults.orgb_address);
                self2.main_list.focus();

                let sec = &self2.main_list.items().text(0);
                match &sec.as_str() {
                    &"defaults" => (),
                    _ => {
                        let section = get_section(&sec);
                        self2.edit_exe.set_text(&section.exe_name);
                        self2.edit_win_name.set_text(&section.game_window_name);
                        self2.edit_ahk_name.set_text(&section.name_ofahk);
                        self2.edit_ahk_path.set_text(&section.path_toahk);
                        self2.edit_orgb_profile.set_text(&section.open_rgbprofile);
                        self2.edit_srgb_profile.set_text(&section.signal_rgbprofile);
                        self2.edit_va_profile.set_text(&section.voice_attack_profile);
                        match section.game_or_win.as_str() {
                            "Game" => {
                                self2.radio_game_win[0].select(true);
                            },
                            _ => {
                                self2.radio_game_win[1].select(true);
                            }
                        };
                        match &section.other_commands.as_str() {
                            &"" => (),
                            s => {
                                let cmds = s.split(" && ");
                                for c in cmds {
                                    self2.cmd_list.items().add(&[c]);
                                }
                            }
                        }
                    }
                };
				Ok(0)
			}
		});

        self.main_list.on().lbn_sel_change({ 
			let self2 = self.clone();
			move || {
                
                let defaults = get_defaults();
                self2.edit_orgb_port.set_text(&defaults.orgb_port);
                self2.edit_orgb_address.set_text(&defaults.orgb_address);

                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                
                match &sec.as_str() {
                    &"defaults" => (),
                    &"Idle" => {
                        let section = get_section(&sec);
                        self2.label_exe.set_text("Idle Time in Seconds");
                        self2.edit_exe.set_text(&section.exe_name);
                        self2.label_win_name.set_text("Night Hours (ie: 2130-0630)");
                        self2.edit_win_name.set_text(&section.game_window_name);
                        self2.edit_ahk_name.set_text(&section.name_ofahk);
                        self2.edit_ahk_path.set_text(&section.path_toahk);
                        self2.edit_orgb_profile.set_text(&section.open_rgbprofile);
                        self2.edit_srgb_profile.set_text(&section.signal_rgbprofile);
                        self2.edit_va_profile.set_text(&section.voice_attack_profile);
                        self2.label_game_win.set_text("Activate Screensaver?");
                        self2.radio_game_win[0].set_text("Yes");
                        self2.radio_game_win[1].set_text("No");
                        match section.game_or_win.as_str() {
                            "Yes" => {
                                self2.radio_game_win[0].select(true);
                                self2.radio_game_win[1].select(false);
                            },
                            _ => {
                                self2.radio_game_win[0].select(false);
                                self2.radio_game_win[1].select(true);
                            }
                        };
                        self2.cmd_list.items().delete_all();
                        match &section.other_commands.as_str() {
                            &"" => (),
                            s => {
                                let cmds = s.split(" && ");
                                for c in cmds {
                                    self2.cmd_list.items().add(&[c]);
                                }
                            }
                        }
                    }
                    _ => {
                        let section = get_section(&sec);
                        self2.label_exe.set_text("Process Name");
                        self2.edit_exe.set_text(&section.exe_name);
                        self2.label_win_name.set_text("Window Name / Title");
                        self2.edit_win_name.set_text(&section.game_window_name);
                        self2.edit_ahk_name.set_text(&section.name_ofahk);
                        self2.edit_ahk_path.set_text(&section.path_toahk);
                        self2.edit_orgb_profile.set_text(&section.open_rgbprofile);
                        self2.edit_srgb_profile.set_text(&section.signal_rgbprofile);
                        self2.edit_va_profile.set_text(&section.voice_attack_profile);
                        self2.label_game_win.set_text("Apply settings on....");
                        self2.radio_game_win[0].set_text("Game");
                        self2.radio_game_win[1].set_text("Window");
                        match section.game_or_win.as_str() {
                            "Game" => {
                                self2.radio_game_win[0].select(true);
                                self2.radio_game_win[1].select(false);
                            },
                            _ => {
                                self2.radio_game_win[0].select(false);
                                self2.radio_game_win[1].select(true);
                            }
                        };
                        self2.cmd_list.items().delete_all();
                        match &section.other_commands.as_str() {
                            &"" => (),
                            s => {
                                let cmds = s.split(" && ");
                                for c in cmds {
                                    self2.cmd_list.items().add(&[c]);
                                }
                            }
                        }

                    }
                };
				Ok(())
			}
		});

        self.radio_game_win.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                if let Some(game_or_win) = self2.radio_game_win.checked() {
                    let gow = game_or_win.hwnd().GetWindowText()?;
                    
                    write_key(&sec, "game-or-win", &gow);
                    msg_box("GameMon", "Saved!");
                }
                Ok(())
            }
        });

        self.btn_cmd_add.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                let section = get_section(&sec);
                let mut final_string = "".to_owned();
                match &section.other_commands.as_str() {
                    &"" => {
                        final_string.push_str(&self2.edit_cmd_add.text());
                        
                        write_key(&sec, "other_commands", &final_string);
                    },
                    s => {
                        let cmds = s.split(" && ");
                        for c in cmds {
                            final_string.push_str(&c);
                        }
                        
                        final_string.push_str(" && ");
                        final_string.push_str(&self2.edit_cmd_add.text());
                        write_key(&sec, "other_commands", &final_string);
                    }
                }
                
                self2.edit_cmd_add.set_text("");
                let section = get_section(&sec);
                self2.cmd_list.items().delete_all();
                match &section.other_commands.as_str() {
                    &"" => (),
                    s => {
                        let cmds = s.split(" && ");
                        for c in cmds {
                            self2.cmd_list.items().add(&[c]);
                        }
                    }
                }

                Ok(())
            }
        });

        self.btn_cmd_delete.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                let section = get_section(&sec);
                let cmd = &self2.cmd_list.items().iter_selected().last().unwrap().1;
                let mut needle = " && ".to_owned();
                needle.push_str(&cmd);
                
                let haystack = &section.other_commands;

                if haystack.contains(&needle) {
                    let final_string = str::replace(&haystack, &needle, "");
                    
                    write_key(&sec, "other_commands", &final_string);
                    self2.edit_cmd_add.set_text("");
                    let section = get_section(&sec);
                    self2.cmd_list.items().delete_all();
                    match &section.other_commands.as_str() {
                        &"" => (),
                        s => {
                            let cmds = s.split(" && ");
                            for c in cmds {
                                self2.cmd_list.items().add(&[c]);
                            }
                        }
                    }
                } else if haystack.contains(&cmd.to_string()) {
                    let final_string = str::replace(&haystack, &cmd.to_string(), "");
                    
                    write_key(&sec, "other_commands", &final_string);
                    self2.edit_cmd_add.set_text("");
                    let section = get_section(&sec);
                    self2.cmd_list.items().delete_all();
                    match &section.other_commands.as_str() {
                        &"" => (),
                        s => {
                            let cmds = s.split(" && ");
                            for c in cmds {
                                self2.cmd_list.items().add(&[c]);
                            }
                        }
                    }
                } else {
                    let final_string = haystack;
                    
                    write_key(&sec, "other_commands", &final_string);
                    self2.edit_cmd_add.set_text("");
                    let section = get_section(&sec);
                    self2.cmd_list.items().delete_all();
                    match &section.other_commands.as_str() {
                        &"" => (),
                        s => {
                            let cmds = s.split(" && ");
                            for c in cmds {
                                self2.cmd_list.items().add(&[c]);
                            }
                        }
                    }
                };

                

                Ok(())
            }
        });

        self.btn_add.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let new = self2.edit_add.text();
                match new.as_str() {
                    "" => (),
                    _ => {
                        self2.edit_add.set_text("");
                        
                        write_section(&new);
                        log!(&format!("Added monitor {}...", &new));

                        self2.main_list.items().delete_all();
        
                        let mut item_vec = Vec::new();
                        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                        let path = Path::new("Software").join("GameMon");
                        let game_mon = hklm.open_subkey(&path).unwrap();
                        
                        for sec in game_mon.enum_keys().map(|x| x.unwrap()){
                            match &sec.as_str() {
                                &"defaults" => (),
                                _ => item_vec.push(sec)
                            }
                        };
        
                        item_vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                        for i in item_vec{
                            self2.main_list.items().add(&[i]);
                        }
                    }
                };
                
                Ok(())
            }
        });

        self.btn_delete.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                delete_section(&sec);
                log!(format!("Deleted monitor {}...", &sec));

                self2.main_list.items().delete_all();

                let mut item_vec = Vec::new();
                let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                let path = Path::new("Software").join("GameMon");
                let game_mon = hklm.open_subkey(&path).unwrap();
                
                for sec in game_mon.enum_keys().map(|x| x.unwrap()){
                    match &sec.as_str() {
                        &"defaults" => (),
                        _ => item_vec.push(sec)
                    }
                };

                item_vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                for i in item_vec{
                    self2.main_list.items().add(&[i]);
                }
                
                
                let defaults = get_defaults();
                self2.edit_orgb_port.set_text(&defaults.orgb_port);
                self2.edit_orgb_address.set_text(&defaults.orgb_address);
                self2.main_list.focus();

                let sec = &self2.main_list.items().text(0);
                match &sec.as_str() {
                    &"defaults" => (),
                    _ => {
                        let section = get_section(&sec);
                        self2.edit_exe.set_text(&section.exe_name);
                        self2.edit_win_name.set_text(&section.game_window_name);
                        self2.edit_ahk_name.set_text(&section.name_ofahk);
                        self2.edit_ahk_path.set_text(&section.path_toahk);
                        self2.edit_orgb_profile.set_text(&section.open_rgbprofile);
                        self2.edit_srgb_profile.set_text(&section.signal_rgbprofile);
                        self2.edit_va_profile.set_text(&section.voice_attack_profile);
                        match section.game_or_win.as_str() {
                            "Game" => {
                                self2.radio_game_win[0].select(true);
                                self2.radio_game_win[1].select(false);
                            },
                            _ => {
                                self2.radio_game_win[0].select(false);
                                self2.radio_game_win[1].select(true);
                            }
                        };

                    }
                };

                Ok(())
            }
        });

        self.btn_save.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = &self2.main_list.items().iter_selected().last().unwrap().1;
                
                write_key(&sec, "exeName", &self2.edit_exe.text());
                write_key(&sec, "gameWindowName", &self2.edit_win_name.text());

                if &self2.edit_ahk_path.text() == "" {
                    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                    let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
                    let mut script_dir: String = g_key.get_value("InstallDir").unwrap();
                    let script_dirname = "\\scripts";
                    script_dir.push_str(&script_dirname);
                    script_dir.push_str(&format!("\\{}.ahk", &sec));

                    File::create(&script_dir).unwrap();
                    let mut lfile = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&script_dir)
                        .unwrap();
                    write!(lfile, "{}", format!("#Persistent\n#SingleInstance, Force\n#NoTrayIcon")).unwrap();

                    self2.edit_ahk_path.set_text(&script_dir);
                }

                write_key(&sec, "nameOfahk", &self2.edit_ahk_name.text());
                write_key(&sec, "pathToahk", &self2.edit_ahk_path.text());
                write_key(&sec, "OpenRGBprofile", &self2.edit_orgb_profile.text());
                write_key(&sec, "voiceAttackProfile", &self2.edit_va_profile.text());
                write_key(&sec, "SignalRGBprofile", &self2.edit_srgb_profile.text());
                if let Some(game_or_win) = self2.radio_game_win.checked() {
                    let gow = game_or_win.hwnd().GetWindowText()?;
                    write_key(&sec, "game-or-win", &gow);
                }
                log!(&format!("Saved settings for {}...", &sec));
                let sec = "defaults".to_string();
                write_key(&sec, "orgb_port", &self2.edit_orgb_port.text());
                write_key(&sec, "orgb_address", &self2.edit_orgb_address.text());
                msg_box("GameMon", "Saved!");
                Ok(())
            }
        });

        self.btn_close.on().bn_clicked({
            let self2 = self.clone();
            move || {
                self2.wnd.hwnd().DestroyWindow()?;
                Ok(())
            }
        });
        
        self.btn_find.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let path2 = FileDialog::new()
                    .set_location(&std::env::current_dir().unwrap().to_str().unwrap().to_string())
                    .show_open_single_file()
                    .unwrap();

                let path_final = match path2 {
                    Some(path) => path.to_str().unwrap().to_string(),
                    None => self2.edit_ahk_path.text(),
                };
                self2.edit_ahk_path.set_text(&path_final);
                Ok(())
            }
        });

    }
}

// Extra Functions

async fn screensaver() -> String{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
    let screen_s: String = desktop.get_value("SCRNSAVE.EXE").unwrap();

    return screen_s;
}

// Change Signal RGB
async fn change_signal_rgb(profile: &String) -> String{
    let sp = &profile;
    let mut rgb_profile = url_encode(sp.to_string());

    if rgb_profile.contains("?"){
        rgb_profile.push_str("^&-silentlaunch-");
    } else {
        rgb_profile.push_str("?-silentlaunch-");
    }
    
    let command_var = format!("start signalrgb://effect/apply/{}", &rgb_profile);
  
    let output = run_cmd(&command_var).await;
    let return_var: String = match output {
        Err(e) => format!("Could not execute SignalRGB Command: {}: {:?}", &command_var, e),
        Ok(_) => format!("Changed SignalRGB to {}", &sp)
    };
    
    sleep(1000).await;
    return return_var;
}

// Change OpenRGB
async fn change_open_rgb(addy: &String, port: &String, profile: &String) -> String {
    let rgb_profile = url_encode(profile.to_string());
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
        reqwest::StatusCode::OK => format!("Changed OpenRGB to {}", &profile),
        reqwest::StatusCode::NO_CONTENT => format!("Changed OpenRGB to {}", &profile),
        e => format!("Could not execute OpenRGB Command: {} Status: {:?}", &command_var, e)
        
    }; 
    
    return return_var.to_string();
}

async fn sleep(milliseconds: u64){
    let mills = std::time::Duration::from_millis(milliseconds);
    tokio::time::sleep(mills).await;
}

fn reset_running() -> String{
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

    write_key("defaults", "gameon", "False");
    write_key("General", "running", "True");
    write_key("General", "running_pid", "0");
    return "Running values reset.".to_string();
}

fn close_all_ahk() -> Result<(), String> {
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon");
    let game_mon = hklm.open_subkey(&path).unwrap();
    
    for sec in game_mon.enum_keys().map(|x| x.unwrap()){

        match &sec.as_str() {
            &"defaults" => (),
            _ => {
                let ahk_pid = ahk_pid(&sec);
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

fn reg_check(){
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut path = Path::new("Software").join("GameMon");
    let disp = hklm.create_subkey(&path).unwrap().1;

    match disp {
        REG_CREATED_NEW_KEY => {
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
                            REG_CREATED_NEW_KEY => {
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
                                write_key(&section_name, "running", "");
                                write_key(&section_name, "window_flag", "General");
                                write_key(&section_name, "screensaver_orgb_profile", "General");
                                write_key(&section_name, "screensaver_srgb_profile", "Screen Ambience");
                            },
                            REG_OPENED_EXISTING_KEY => {
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
        
            section_name = "Idle".to_string();
            write_key(&section_name, "exeName", "300");
            write_key(&section_name, "gameWindowName", "2100-0600");
            write_key(&section_name, "game-or-win", "Game");
        },
        REG_OPENED_EXISTING_KEY => {
            log!(&"An existing key has been opened".to_string());
        },
    }

    

    
}

fn reg_write_value(path: &PathBuf, name: String, value: String) -> Result<(), std::io::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.create_subkey(&path).unwrap().0;

    return key.set_value(&name, &value);
}

fn reg_section_new(sec: String) {
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



fn initialize_log(){
    let now = timestamp();
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
    let mut log_file: String = g_key.get_value("InstallDir").unwrap();
    let mut log_dir: String = g_key.get_value("InstallDir").unwrap();
    let mut log_archive: String = g_key.get_value("InstallDir").unwrap();
    let mut script_dir: String = g_key.get_value("InstallDir").unwrap();

    let filename: &str= "\\gamemon.log";
    let dirname: &str = "\\logs";
    let script_dirname: &str = "\\scripts";
    log_file.push_str(&filename);
    log_dir.push_str(&dirname);
    log_archive.push_str(&dirname);
    script_dir.push_str(&script_dirname);
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

    let s = std::path::Path::new(&script_dir).exists();
    if s {
        
    } else {
        std::fs::create_dir(&script_dir).expect("Could not create scripts directory!");
    }
    
}

fn timestamp() -> String {
    let mut dt = Local::now().date().format("%Y%m%d").to_string();
    dt.push_str(&Local::now().time().format("%H%M%S").to_string());
    return dt
}

fn msg_box(title: &str, text: &str){
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(&title)
        .set_text(&format!("{}", &text))
        .show_alert()
        .unwrap();
}

fn get_pid(pname: Option<&str>) -> Result<u32, &str>{
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

fn ahk_pid(sec: &String) -> Result<u32, String> {
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

fn name_by_pid(pid: Pid) -> Result<String, String>{
    let s = System::new_all();
    if let Some(process) = s.process(pid){
        return Ok(process.name().to_owned());
    }

    return Err("None".to_string());
}

fn get_section(sec_name: &String) -> Instance {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut path = Path::new("Software").join("GameMon");
    let game_mon = hklm.open_subkey(&path).unwrap();
    let mut section = Instance::new();
    
    for i in game_mon.enum_keys().map(|x| x.unwrap()).filter(|x| x.contains(sec_name)) {
        path = Path::new("Software").join("GameMon").join(i);
        let sec = hklm.open_subkey(&path).unwrap();

        for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
            match name.as_str() {
                // "exe_name" => section.exe_name = value.to_string(), 
                "exe_name" => section.exe_name = value.to_string(),
                "game_or_win" => section.game_or_win = value.to_string(),
                "game_window_name" => section.game_window_name = value.to_string(),
                "name_ofahk" => section.name_ofahk = value.to_string(),
                "open_rgbprofile" => section.open_rgbprofile = value.to_string(),
                "other_commands" => section.other_commands = value.to_string(),
                "path_toahk" => section.path_toahk = value.to_string(),
                "running" => section.running = value.to_string(),
                "running_pid" => section.running_pid = value.to_string(),
                "signal_rgbprofile" => section.signal_rgbprofile = value.to_string(),
                "voice_attack_profile" => section.voice_attack_profile = value.to_string(),
                _ => ()
            }

        }
    }

    return section
}

fn get_defaults() -> Defaults {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut path = Path::new("Software").join("GameMon");
    let game_mon = hklm.open_subkey(&path).unwrap();
    let mut section = Defaults::new();
    
    for i in game_mon.enum_keys().map(|x| x.unwrap()).filter(|x| x.contains("defaults")) {
        path = Path::new("Software").join("GameMon").join(i);
        let sec = hklm.open_subkey(&path).unwrap();

        for (name, value) in sec.enum_values().map(|x| x.unwrap()) {
            match name.as_str() {
                "default_orgb_profile" => section.default_orgb_profile = value.to_string(),
                "default_srgb_profile" => section.default_srgb_profile = value.to_string(),
                "exit_reason" => section.exit_reason = value.to_string(),
                "gameon" => section.gameon = value.to_string(),
                "night_hour_orgb_profile" => section.night_hour_orgb_profile = value.to_string(),
                "night_hour_srgb_profile" => section.night_hour_srgb_profile = value.to_string(),
                "openrgb_path" => section.openrgb_path = value.to_string(),
                "orgb_address" => section.orgb_address = value.to_string(),
                "orgb_port" => section.orgb_port = value.to_string(),
                "screensaver_orgb_profile" => section.screensaver_orgb_profile = value.to_string(),
                "screensaver_srgb_profile" => section.screensaver_srgb_profile = value.to_string(),
                "voice_attack_path" => section.voice_attack_path = value.to_string(),
                "window_flag" => section.window_flag = value.to_string(),
                _ => ()
            }

        }
    }

    return section
}

fn write_key(sec_name: &str, key_name: &'static str, key_value: &str){
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(sec_name);
    let key = hklm.create_subkey(&path).unwrap().0;

    return key.set_value(&key_name, &key_value).unwrap();
}

fn write_section(sec_name: &String){
    write_key(&sec_name, "exeName", "");
    write_key(&sec_name, "gameWindowName", "");
    write_key(&sec_name, "nameOfahk", "");
    write_key(&sec_name, "pathToahk", "");
    write_key(&sec_name, "OpenRGBprofile", "");
    write_key(&sec_name, "voiceAttackProfile", "");
    write_key(&sec_name, "SignalRGBprofile", "");
    write_key(&sec_name, "game-or-win", "");
    write_key(&sec_name, "running", "");
    write_key(&sec_name, "running_pid", "");
}

fn delete_section(sec_name: &String){
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(sec_name);
    hklm.delete_subkey_all(path).unwrap();
}

fn url_encode(data: String) -> String{
    let data = str::replace(&data, "\n", "%0A");
    let data = str::replace(&data, "+", "%2b");
    let data = str::replace(&data, "\r", "%0D");
    let data = str::replace(&data, "'", "%27");
    let data = str::replace(&data, " ", "%20");
    let data = str::replace(&data, "#", "%23");
    let data = str::replace(&data, "&", "^&");
    return data;
}

fn test(){

    
}

async fn run_cmd(cmd: &String) -> Result<std::process::Child, std::io::Error>{
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg(&cmd)
        .spawn();
    
    return output
}

fn close_pid(pid: u32) -> Result<std::process::Child, std::io::Error>{
    // let win_pid = get_pid(Some("Autohotkey.exe")).unwrap();
    // let win_hwnd = windows_win::raw::window::get_by_pid(&win_pid).unwrap();
    // let hwnd = windows_win::raw::window::get_by_title(&ahk_name, &win_hwnd.unwrap());

    let kill_cmd = format!("TASKKILL /PID {}", &pid);
    let output = Command::new("cmd.exe")
    .creation_flags(CREATE_NO_WINDOW)
    .arg("/c")
    .arg(&kill_cmd)
    .spawn();

return output
}

async fn defaults_gui(){
    let my = DefaultsWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}

async fn main_gui(){
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}

#[cfg(windows)]
#[tokio::main]
async fn main() {
    // Initialize Setup
    reg_check();
    initialize_log();
    let _cleanup = Cleanup;

    let defaults = get_defaults();
    log!(format!("Last shutdown reason: {}", &defaults.exit_reason));

    // Create system tray
    let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

    tray.add_label("GameMon").unwrap();
    
    tray.add_menu_item("About", || {
        msg_box("About", &format!("GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language").to_string());
    })
    .unwrap();
    
    let (tx, rx) = mpsc::channel();
    let txc = tx.clone();
    
    tray.add_menu_item("Monitors", move || {
        println!("GUI");
        txc.send(Message::Gui).unwrap();
    }).unwrap();
    
    let txc = tx.clone();

    
    tray.add_menu_item("Default Settings", move || {
        println!("Default Settings");
        txc.send(Message::Defaults).unwrap();
    }).unwrap();
    
    let txc = tx.clone();


    tray.add_menu_item("Quit", move || {
        println!("Quit");
        txc.send(Message::Quit).unwrap();
    })
    .unwrap();

    /////////// test zone //////////////////
    test();
    ////////////////////////////////////////

    // Initialize Loop variables

    // Read INI Sections and operate on each one
    loop {

        match rx.try_recv(){
            Ok(Message::Quit) => exit_app!(1, "Menu"),
            Ok(Message::Gui) => main_gui().await,
            Ok(Message::Defaults) => defaults_gui().await,
            _ => ()
        };

        let s = System::new_all();
        let procs = s.processes_by_exact_name("GameMon.exe");

        for process in procs{
            match process.memory().cmp(&"1000000000".parse::<u64>().unwrap()){
                Ordering::Greater => {
                    exit_app!(0, "Memory allocation too high");
                    
                },
                _ => ()
            }
            
        }

        // Game and Window Reactions
        let defaults = get_defaults();

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("GameMon");
        let game_mon = hklm.open_subkey(&path).unwrap();
        
        for sec in game_mon.enum_keys().map(|x| x.unwrap()){

            let section = match &sec.as_str() {
                &"defaults" => continue,
                &"General" => continue,
                _ => get_section(&sec),
            };

            match &sec.as_str() {
                &"Idle" => { // Begin Idle Reaction
                    log!(&section.exe_name, "d");

                    let idle_wait = &section.exe_name.parse::<u64>().unwrap();
                    let idle_seconds = UserIdle::get_time().unwrap().as_seconds();
                    let time_of_day = Local::now().time();
                    let time_range = &section.game_window_name.split("-").collect::<Vec<&str>>();
                    let start_time = NaiveTime::parse_from_str(time_range[0], "%H%M").unwrap();
                    let end_time = NaiveTime::parse_from_str(time_range[1], "%H%M").unwrap();

                    match idle_seconds.cmp(&idle_wait){
                        Ordering::Greater => { // PAST IDLE TIME!!!!!!
                            
                            match &section.running.as_str() {
                                &"False" => { //IDLE IS NOT RUNNING
                                    //change values
                                    
                                    write_key("defaults", "gameon", "True");
                                    write_key(&sec, "running", "True");
                                    write_key("General", "running", "False");

                                    if (time_of_day > start_time) || (time_of_day < end_time) { // PAST DARK HOURS!!!!!
                                        log!(&format!("Idle detected! After dark hours!"));

                                        //change profiles
                                        log!(change_signal_rgb(&defaults.night_hour_srgb_profile).await);
                                        log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.night_hour_orgb_profile).await);

                                        // Run other commands                                        
                                        match windows_win::raw::window::send_message(
                                            windows_win::raw::window::get_by_class("Progman", None).unwrap()[0], 0x112, 0xF170, 2 , Some(5)) {
                                                Ok(_) => (),
                                                Err(e) => log!(&format!("Failed to turn off monitor(s)!! || Error: {}", &e), "e")
                                        };

                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                for c in s.split(" && ") {
                                                    let r = run_cmd(&c.to_string()).await;
                                                    match r {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    }
                                                }
                                            }
                                        }

                                    } else { // NOT PAST DARK HOURS!!!!
                                        log!("Idle detected! Within Day Hours.");

                                        //change values
                                        write_key("defaults", "gameon", "True");
                                        write_key("General", "running", "False");

                                        // Run Screensaver
                                        match section.game_or_win.as_str() {
                                            "Yes" => {
                                                log!("Running Screensaver...");
                                                //change profiles
                                                let signal_out = change_signal_rgb(&defaults.screensaver_srgb_profile).await;
                                                let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.screensaver_orgb_profile).await;
                                                log!(&signal_out);
                                                log!(&open_out);
                                                let ss = format!("{} /S", screensaver().await);
                                                match run_cmd(&ss).await {
                                                    Ok(_) => (),
                                                    Err(e) => log!(&format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                };
                                            },
                                            _ => {
                                                //change profiles
                                                let signal_out = change_signal_rgb(&section.signal_rgbprofile).await;
                                                let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).await;
                                                log!(&signal_out);
                                                log!(&open_out);
                                            },
                                        }

                                        // Run other commands
                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                let cmds = s.split(" && ");
                                                for c in cmds {
                                                    let r = run_cmd(&c.to_string()).await;
                                                    match r {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    }
                                                }
                                            }
                                        }
                                        
                                    };
                                    let ahk_run = run_cmd(&section.path_toahk).await;
                                    assert!(ahk_run.is_ok());
                                    log!(&format!("{} is running!", section.name_ofahk));

                                },
                                _ => { // Idle is running!
                                    if (time_of_day > start_time) || (time_of_day < end_time) { // PAST DARK HOURS!!!!!

                                    } else {
                                        let ss_exe = screensaver().await;
                                        match get_pid(Some(&ss_exe)) { // Check for Screensaver
                                            Ok(_) => (),
                                            _ => {
                                                log!(&format!("Idle is running but screensaver not detected!"));
                                                
                                                // Run Screensaver
                                                match section.game_or_win.as_str() {
                                                    "Yes" => {
                                                        //change profiles
                                                        log!("Running Screensaver...");
                                                        let signal_out = change_signal_rgb(&defaults.screensaver_srgb_profile).await;
                                                        let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.screensaver_orgb_profile).await;
                                                        log!(&signal_out);
                                                        log!(&open_out);
                                                        let ss = format!("{} /S", screensaver().await);
                                                        match run_cmd(&ss).await {
                                                            Ok(_) => (),
                                                            Err(e) => log!(&format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                        };
                                                    },
                                                    _ => {
                                                        //change profiles
                                                        let signal_out = change_signal_rgb(&section.signal_rgbprofile).await;
                                                        let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).await;
                                                        log!(&signal_out);
                                                        log!(&open_out);
                                                    },
                                                }
                                            }
                                        };
                                    };

                                    

                                }
                            }
                        },
                        _ => { // NOT PAST IDLE TIME
                            match &section.running.as_str() {
                                &"True" => { //IDLE IS RUNNING
                                    log!(&format!("Idle no longer detected!"));

                                    //change values
                                    write_key("General", "running", "True");
                                    write_key(&sec, "running", "False");
                                    write_key("defaults", "gameon", "False");

                                    //run extra commands
                                    let ahk_pid = ahk_pid(&sec.to_string());
                                    assert!(ahk_pid.is_ok());
                                    let ahk_pid = ahk_pid.unwrap();

                                    let ahk_close = close_pid(ahk_pid);
                                    assert!(ahk_close.is_ok());
                                    log!(&format!("{} is no longer running!", section.name_ofahk));
                                    
                                    log!(&format!("{}", reset_running()));
                                    
 
                                },
                                _ => ()
                            }
                        }
                    };
                }, // End Idle Reaction

                _ => { // Begin ALL OTHER Reaction

                    match &section.exe_name.as_str() {
                        &"" => (),
                        _ => ()
                    }

                    match section.game_or_win.as_str() {
                        "Game" => {
                            
                            //is program running?
                            let game_bool = get_pid(Some(&section.exe_name));
                            match game_bool {
                                Ok(0) => { //Program not found
                                    match &section.running.as_str() {
                                        &"True" => { //Profile is on
                                            log!(&format!("{} no longer detected!", &section.exe_name));

                                            //change values
                                            write_key("General", "running", "True");
                                            write_key(&sec, "running", "False");
                                            write_key("defaults", "gameon", "False");

                                            //run extra commands
                                            let ahk_pid = ahk_pid(&sec.to_string());
                                            assert!(ahk_pid.is_ok());
                                            let ahk_pid = ahk_pid.unwrap();

                                            let ahk_close = close_pid(ahk_pid);
                                            assert!(ahk_close.is_ok());
                                            log!(&format!("{} is no longer running!", section.name_ofahk));
                                            
                                            log!(&format!("{}", reset_running()));   
                                            
                                        },
                                        _ => (),
                                    }; 
                                },
                                Ok(msg) => { //Program Found
                                    match &section.running.as_str() {
                                        &"False" => { // Profile Off
                                            log!(&format!("{} detected!", &section.exe_name));

                                            //change values
                                            write_key(&sec, "running", "True");
                                            write_key(&sec, "running_pid", &msg.to_string());
                                            write_key("General", "running", "False");
                                            write_key("defaults", "gameon", "True");

                                            //change profiles
                                            let signal_out = change_signal_rgb(&section.signal_rgbprofile).await;
                                            let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).await;
                                            log!(&signal_out);
                                            log!(&open_out);
                                            match &section.voice_attack_profile.as_str() {
                                                &"" => (),
                                                _ => {
                                                    let r = Command::new(&defaults.voice_attack_path)
                                                        .creation_flags(CREATE_NO_WINDOW)
                                                        .arg("-profile")
                                                        .arg(&section.voice_attack_profile)
                                                        .spawn();
                                                    match r {
                                                        Ok(_) => {
                                                            log!(format!("Changed VoiceAttack profile to {}", &section.voice_attack_profile));
                                                            
                                                        },
                                                        Err(e) => log!(format!("Could not change VoiceAttack profile: {}", &e), "e"),
                                                    }
                                                }
                                            };

                                            //run extra commands
                                            let ahk_run = run_cmd(&section.path_toahk).await;
                                            assert!(ahk_run.is_ok());
                                            log!(&format!("{} is running!", &section.name_ofahk));
                                            match &section.other_commands.as_str() {
                                                &"" => (),
                                                s => {
                                                    let cmds = s.split(" && ");
                                                    for c in cmds {
                                                        let r = run_cmd(&c.to_string()).await;
                                                        match r {
                                                            Ok(_) => log!(format!("Running {}", &c)),
                                                            Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            
                                        },
                                        _ => ()
                                    };
                                },
                                _ => {
                                    
                                }
                            };


                        },
                        "Window" => {
                            // is a game running?
                            match &defaults.gameon.as_str() {
                                &"True" => continue,
                                _ => ()
                            };

                            let win_flag = &defaults.window_flag;

                            // is window active?
                            let active_pid = get_active_window().unwrap().process_id;
                            let active_win = name_by_pid(Pid::from(active_pid as usize)).unwrap();

                            if &active_win == &section.exe_name{ // ** WINDOW IS ACTIVE **
                                match &section.running.as_str() {
                                    &"False" => { //Profile Off
                                        log!(&format!("{} detected!", &section.exe_name));
                                        
                                        //change values
                                        write_key(&sec, "running", "True");
                                        write_key(&sec, "running_pid", &active_pid.to_string());
                                        write_key("General", "running", "False");
                                        write_key("defaults", "window_flag", &sec);

                                        //change profiles
                                        let signal_out = change_signal_rgb(&section.signal_rgbprofile).await;
                                        let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).await;
                                        log!(&signal_out);
                                        log!(&open_out);
                                        match &section.voice_attack_profile.as_str() {
                                            &"" => (),
                                            _ => {
                                                let r = Command::new(&defaults.voice_attack_path)
                                                    .creation_flags(CREATE_NO_WINDOW)
                                                    .arg("-profile")
                                                    .arg(&section.voice_attack_profile)
                                                    .spawn();
                                                match r {
                                                    Ok(_) => {
                                                        log!(format!("Changed VoiceAttack profile to {}", &section.voice_attack_profile));
                                                        
                                                    },
                                                    Err(e) => log!(format!("Could not change VoiceAttack profile: {}", &e), "e"),
                                                }
                                            }
                                        };

                                        //run extra commands
                                        let ahk_run = run_cmd(&section.path_toahk).await;
                                        assert!(ahk_run.is_ok());
                                        log!(&format!("{} is running!", &section.name_ofahk));
                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                let cmds = s.split(" && ");
                                                for c in cmds {
                                                    let r = run_cmd(&c.to_string()).await;
                                                    match r {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    }
                                                }
                                            }
                                        }
                                        
                                        
                                    },
                                    _ => {
                                        if &win_flag == &&sec {
                                        } else {
                                            write_key("defaults", "window_flag", &sec);
                                            write_key("General", "running", "False");

                                            //change profiles
                                            let signal_out = change_signal_rgb(&section.signal_rgbprofile).await;
                                            let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).await;
                                            log!(&signal_out);
                                            log!(&open_out);
                                        }
                                    }
                                };
                            } else { // ** WINDOW IS NOT ACTIVE **
                                match &section.running.as_str() {
                                    &"True" => { // Profile On
                                        log!(&format!("{} no longer detected!", &section.exe_name));

                                        //change values
                                        write_key(&sec, "running", "False");
                                        if &win_flag == &&sec {
                                            write_key("defaults", "window_flag", "General");
                                            write_key("General", "running", "True");
                                        
                                        }

                                        //run extra commands
                                        let ahk_pid = ahk_pid(&sec.to_string());
                                        assert!(ahk_pid.is_ok());
                                        let ahk_pid = ahk_pid.unwrap();

                                        let ahk_close = close_pid(ahk_pid);
                                        assert!(ahk_close.is_ok());
                                        log!(&format!("{} is no longer running!", section.name_ofahk));

                                    },
                                    _ => {
                                        if &win_flag == &&sec {
                                            write_key("defaults", "window_flag", "General");
                                            
                                            
                                        }
                                    }
                                };
                            }
                        }
                        _ => ()
                    };
                } //End Game or Window Reaction

            }; // End of section match

            let general = get_section(&"General".to_string());
            
            if &general.running == "True" {
                if &general.running_pid == "0" {
                    //change profiles
                    let signal_out = change_signal_rgb(&general.signal_rgbprofile).await;
                    let open_out = change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &general.open_rgbprofile).await;
                    log!(&signal_out);
                    log!(&open_out);
                    match &general.voice_attack_profile.as_str() {
                        &"" => (),
                        _ => {
                            let r = Command::new(&defaults.voice_attack_path)
                                .creation_flags(CREATE_NO_WINDOW)
                                .arg("-profile")
                                .arg(&general.voice_attack_profile)
                                .spawn();
                            match r {
                                Ok(_) => {
                                    log!(format!("Changed VoiceAttack profile to {}", &general.voice_attack_profile));
                                    
                                },
                                Err(e) => log!(format!("Could not change VoiceAttack profile: {}", &e), "e"),
                            }
                        }
                    };
                    
                    write_key("General", "running_pid", "1");
                } 
            } else {
                match general.running_pid.as_str() {
                    "0" => (),
                    _ => write_key("General", "running_pid", "0"),
                };
            };
        }; // End of section "For" loop
        sleep(1000).await;
    };
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
