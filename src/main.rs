// Rust Programming Language
// #####################################################################
// File: main.rs                                                       #
// Project: src                                                        #
// Created Date: Mon, 12 Sep 2022 @ 20:09:15                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 10 Dec 2022 @ 11:38:04                          #
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
use native_dialog::{FileDialog};
use active_win_pos_rs::get_active_window;
use std::{process::{Command}, os::windows::{process::CommandExt}, io::Write, fs::{OpenOptions, File}, path::{Path, PathBuf}, cmp::Ordering};
use chrono::{Local, NaiveTime};
use {std::sync::mpsc, tray_item::TrayItem};
use winreg::{enums::*};
use winreg::RegKey;
use windows_win::{raw::window::{
    get_by_title,
    get_thread_process_id,
    send_message,
    get_by_class
}};
use winapi::{
    um::{
        winuser::{
            LASTINPUTINFO,
            PLASTINPUTINFO,
            GetLastInputInfo, BM_CLICK, GetDesktopWindow
        },
    }};
use winsafe::{prelude::*, WString, co::{DLGID, HRESULT}};
use winsafe::{HWND, gui, POINT, SIZE, co::{COLOR, WS, WS_EX, SS, MB}};
use quoted_string;
use quoted_string::strip_dquotes;
use ureq::{self};
use ureq::Error;
use eventlog;
use log::{info, trace, debug, error, warn};
use mouse_rs::{types::keys::Keys, Mouse};
use msgbox;

// Environment Variables
const CREATE_NO_WINDOW: u32 = 0x08000000;

// Macros
macro_rules! d_quote {
    ($a:expr) => {
        strip_dquotes($a).unwrap().to_string()
    }
}

macro_rules! exit_app {
    
    ($a:expr) => {
        {
            log!("Exiting.  Reason: Shutdown", "w");
            log!(format!("{}", reset_running()), "w");
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log!(format!("All ahk scripts are closed"), "w");

            write_key(&"defaults".to_string(), "exit_reason", "Shutdown");

            eventlog::deregister("GameMon Log").unwrap();
            
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

            write_key(&"defaults".to_string(), "exit_reason", $b);

            eventlog::deregister("GameMon Log").unwrap();
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

            write_key(&"defaults".to_string(), "exit_reason", $b);

            eventlog::deregister("GameMon Log").unwrap();
            std::process::exit($a);
        }
    };
    
    () => {
        {
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            eventlog::deregister("GameMon Log").unwrap();
            std::process::abort();
        }
    };
}

macro_rules! log {
    ($a:expr) => {
        {
            info!("{}", $a);
            $a
        }
    };

    ($a:expr,$b:expr) => {
        {
            match $b {
                "i" => {
                    info!("{}", $a);
                    $a
                },
                "d" => {
                    debug!("{}", $a);
                    $a                   
                },
                "e" => {
                    error!("{}", $a);
                    $a                    
                },
                "w" => {
                    warn!("{}", $a);
                    $a                    
                },
                "t" => {
                    trace!("{}", $a);
                    $a                    
                },
                _ => $a,
            }
                   
        }
    };

    () => {
        {
            trace!("{}", "BREAK BREAK BREAK ----------------");
            $a          
        }
    }
}

// Enums, Structs, Impl

#[derive(Debug)]
enum Message {
    Quit,
    Gui,
    Defaults,
    Logs,
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
    other_commands: String,
    priority: String
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
            priority: "".to_string()
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
    window_flag: String,
    current_priority: String
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
            current_priority: "".to_string()
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
                self2.label_openRGBpath.set_text("OpenRGB Path to executable");
                self2.label_voiceAttackPath.set_text("VoiceAttack Path to executable");
                self2.label_defaultORGBProfile.set_text("Default OpenRGB Profile");
                self2.label_defaultSRGBProfile.set_text("Default SignalRGB Profile");
                self2.label_screensaver_orgb_profile.set_text("Default OpenRGB Profile for Screensaver");
                self2.label_screensaver_srgb_profile.set_text("Default SignalRGB Profile for Screensaver");
                self2.label_night_hour_orgb_profile.set_text("Default OpenRGB Profile for Night Hours");
                self2.label_night_hour_srgb_profile.set_text("Default SignalRGB Profile for Night Hours");
                self2.label_orgb_port.set_text("OpenRGB port");
                self2.label_orgb_address.set_text("OpenRGB Address");
              
                    
				Ok(0)
			}
		});

        self.btn_save.on().bn_clicked({
            let self2 = self.clone();
            move || {
                write_key(&"defaults".to_string(), "openRGBPath", self2.edit_openRGBpath.text().as_str());
                write_key(&"defaults".to_string(), "voiceAttackPath", self2.edit_voiceAttackPath.text().as_str());
                write_key(&"defaults".to_string(), "defaultORGBProfile", self2.edit_defaultORGBProfile.text().as_str());
                write_key(&"defaults".to_string(), "defaultSRGBProfile", self2.edit_defaultSRGBProfile.text().as_str());
                write_key(&"defaults".to_string(), "screensaver_orgb_profile", self2.edit_screensaver_orgb_profile.text().as_str());
                write_key(&"defaults".to_string(), "screensaver_srgb_profile", self2.edit_screensaver_srgb_profile.text().as_str());
                write_key(&"defaults".to_string(), "night_hour_orgb_profile", self2.edit_night_hour_orgb_profile.text().as_str());
                write_key(&"defaults".to_string(), "night_hour_srgb_profile", self2.edit_night_hour_srgb_profile.text().as_str());
                write_key(&"defaults".to_string(), "orgb_port", self2.edit_orgb_port.text().as_str());
                write_key(&"defaults".to_string(), "orgb_address", self2.edit_orgb_address.text().as_str());
                match msg_box("GameMon - SAVED!".to_string(), "Saved!".to_string(), 1000) {
                    Ok(o) => {
                        log!(&format!("Saved settings for {}...\n\n{}", &"defaults".to_string(), o));
                    },
                    Err(e) => {
                        log!(&format!("Saved settings for {}...\n\nERROR: Error closing dialog {}", &"defaults".to_string(), e), "w");
                    }
                }
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
    label_srgb_profile: gui::Label,
    edit_srgb_profile: gui::Edit,
    label_va_profile: gui::Label,
    edit_va_profile: gui::Edit,
    label_game_win: gui::Label,
    btn_save: gui::Button,
    radio_game_win: gui::RadioGroup,
    label_priority: gui::Label,
    radio_priority: gui::RadioGroup,
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
                ex_style: WS_EX::TRANSPARENT,
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

        let label_priority = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Priority....".to_owned(),
                size: SIZE::new(400,20),
                position: POINT::new(570, last_y),
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

        let radio_priority = gui::RadioGroup::new(
            &wnd, &[
                gui::RadioButtonOpts {
                    text: "1".to_owned(),
                    selected: false,
                    position: POINT::new(570, last_y + 2),  
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "2".to_owned(),
                    selected: false,
                    position: POINT::new(620, last_y + 2),  
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "3".to_owned(),
                    selected: false,
                    position: POINT::new(670, last_y + 2),  
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "4".to_owned(),
                    selected: false,
                    position: POINT::new(720, last_y + 2),  
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
            label_priority,
            radio_priority,
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

                self2.label_ahk_name.set_text("Name of AHK Script to run");
                self2.label_ahk_path.set_text("Path to AHK Script to run");
                self2.label_orgb_profile.set_text("OpenRGB Profile to Apply (***REQUIRES OpenRGB Webhooks Plugin***)");
                self2.label_srgb_profile.set_text("SignalRGB Profile to Apply");
                self2.label_va_profile.set_text("VoiceAttack Profile to Apply");
                self2.label_other_commands.set_text("------------------------------------------------------------- \
                                Other Commands -------------------------------------------------------------");
                self2.label_priority.set_text("Priority....");
                
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

                self2.main_list.focus();

                let sec = self2.main_list.items().text(0);
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
                        match section.priority.as_str() {
                            "1" => {
                                self2.radio_priority[0].select(true);
                            },
                            "2" => {
                                self2.radio_priority[1].select(true);
                            },
                            "3" => {
                                self2.radio_priority[2].select(true);
                            },
                            _ => {
                                self2.radio_priority[3].select(true);
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
            
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
                
                match &sec.as_str() {
                    &"defaults" => (),
                    &"Idle" => {
                        let section = get_section(&sec);
                        self2.label_exe.set_text("Idle Time in Seconds");
                        self2.edit_exe.set_text(&ss_get("ScreenSaveTimeOut"));
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
                        match ss_get("ScreenSaveActive").as_str() {
                            "1" => {
                                self2.radio_game_win[0].select(true);
                                self2.radio_game_win[1].select(false);
                            },
                            _ => {
                                self2.radio_game_win[0].select(false);
                                self2.radio_game_win[1].select(true);
                            }
                        };
                        match section.priority.as_str() {
                            "1" => {
                                self2.radio_priority[0].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            "2" => {
                                self2.radio_priority[1].select(true);
                                self2.radio_priority[0].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            "3" => {
                                self2.radio_priority[2].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[0].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            _ => {
                                self2.radio_priority[3].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[0].select(false);
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
                        match section.priority.as_str() {
                            "1" => {
                                self2.radio_priority[0].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            "2" => {
                                self2.radio_priority[1].select(true);
                                self2.radio_priority[0].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            "3" => {
                                self2.radio_priority[2].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[0].select(false);
                                self2.radio_priority[3].select(false);
                            },
                            _ => {
                                self2.radio_priority[3].select(true);
                                self2.radio_priority[1].select(false);
                                self2.radio_priority[2].select(false);
                                self2.radio_priority[0].select(false);
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
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
                if let Some(game_or_win) = self2.radio_game_win.checked() {
                    let gow = game_or_win.hwnd().GetWindowText()?;
                    
                    write_key(&sec, "game_or_win", &gow);
                    match &sec.as_str() {
                        &"Idle" => {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        },
                        _ => ()
                    }

                    match msg_box("GameMon - SAVED!".to_string(), "Saved!".to_string(), 1000) {
                        Ok(o) => {
                            log!(&format!("Saved settings for {}...\n\n{}", &sec, o));
                        },
                        Err(e) => {
                            log!(&format!("Saved settings for {}...\n\nERROR: Error closing dialog {}", &sec, e), "w");
                        }
                    }
                    
                }
                reset_running();

                Ok(())
            }
        });

        self.radio_priority.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
                if let Some(priority) = self2.radio_priority.checked() {
                    let gow = priority.hwnd().GetWindowText()?;
                    
                    write_key(&sec, "priority", &gow);
                    match msg_box("GameMon - SAVED!".to_string(), "Saved!".to_string(), 1000) {
                        Ok(o) => {
                            log!(&format!("Saved settings for {}...\n\n{}", &sec, o));
                        },
                        Err(e) => {
                            log!(&format!("Saved settings for {}...\n\nERROR: Error closing dialog {}", &sec, e), "w");
                        }
                    }
                }
                reset_running();

                Ok(())
            }
        });

        self.btn_cmd_add.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
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
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
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
                reset_running();
                
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
                
                self2.main_list.focus();

                let sec = self2.main_list.items().text(0);
                match sec.as_str() {
                    "defaults" => (),
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
                reset_running();
                Ok(())
            }
        });

        self.btn_save.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
                
                write_key(&sec, "exe_name", &self2.edit_exe.text());

                match &sec.as_str() {
                    &"Idle" => {
                        ss_set("ScreenSaveTimeOut", &self2.edit_exe.text());
                    },
                    _ => ()
                }

                write_key(&sec, "game_window_name", &self2.edit_win_name.text());

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

                write_key(&sec, "name_ofahk", &self2.edit_ahk_name.text());
                write_key(&sec, "path_toahk", &self2.edit_ahk_path.text());
                write_key(&sec, "open_rgbprofile", &self2.edit_orgb_profile.text());
                write_key(&sec, "voice_attack_profile", &self2.edit_va_profile.text());
                write_key(&sec, "signal_rgbprofile", &self2.edit_srgb_profile.text());
                if let Some(game_or_win) = self2.radio_game_win.checked() {
                    let gow = game_or_win.hwnd().GetWindowText()?;
                    write_key(&sec, "game-or-win", &gow);
                    match &sec.as_str() {
                        &"Idle" => {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        },
                        _ => ()
                    }
                }
                if let Some(priority) = self2.radio_priority.checked() {
                    let gow = priority.hwnd().GetWindowText()?;
                    write_key(&sec, "priority", &gow);
                }
                log!(&format!("Saved settings for {}...", &sec));
                match msg_box("GameMon - SAVED!".to_string(), "Saved!".to_string(), 1000) {
                    Ok(o) => {
                        log!(&format!("Saved settings for {}...\n\n{}", &sec, o));
                    },
                    Err(e) => {
                        log!(&format!("Saved settings for {}...\n\nERROR: Error closing dialog {}", &sec, e), "w");
                    }
                }
                reset_running();
                Ok(())
            }
        });

        self.btn_close.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let sec = self2.main_list.items().iter_selected().last().unwrap().1;
                
                write_key(&sec, "exe_name", &self2.edit_exe.text());

                match &sec.as_str() {
                    &"Idle" => {
                        ss_set("ScreenSaveTimeOut", &self2.edit_exe.text());
                    },
                    _ => ()
                }

                write_key(&sec, "game_window_name", &self2.edit_win_name.text());

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

                write_key(&sec, "name_ofahk", &self2.edit_ahk_name.text());
                write_key(&sec, "path_toahk", &self2.edit_ahk_path.text());
                write_key(&sec, "open_rgbprofile", &self2.edit_orgb_profile.text());
                write_key(&sec, "voice_attack_profile", &self2.edit_va_profile.text());
                write_key(&sec, "signal_rgbprofile", &self2.edit_srgb_profile.text());
                if let Some(game_or_win) = self2.radio_game_win.checked() {
                    let gow = game_or_win.hwnd().GetWindowText()?;
                    write_key(&sec, "game-or-win", &gow);
                    match &sec.as_str() {
                        &"Idle" => {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        },
                        _ => ()
                    }
                }
                if let Some(priority) = self2.radio_priority.checked() {
                    let gow = priority.hwnd().GetWindowText()?;
                    write_key(&sec, "priority", &gow);
                }
                log!(&format!("Saved settings for {}...", &sec));
                match msg_box("GameMon - SAVED!".to_string(), "Saved!".to_string(), 1000) {
                    Ok(o) => {
                        log!(&format!("Saved settings for {}...\n\n{}", &sec, o));
                    },
                    Err(e) => {
                        log!(&format!("Saved settings for {}...\n\nERROR: Error closing dialog {}", &sec, e), "w");
                    }
                }
                self2.wnd.hwnd().DestroyWindow()?;
                reset_running();
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

fn msg_box(title: String, message: String, exit_wait_time_in_ms: u64) -> Result<DLGID, HRESULT> {
    let msg = message.clone();
    let til = title.clone();
    let y = std::thread::spawn(move ||{
        let _rr = msgbox::create(&til, &msg, msgbox::IconType::Info);

    });
    
    let f = match exit_wait_time_in_ms {
        0 => Ok(DLGID::OK),
        _ => {
            sleep(exit_wait_time_in_ms);

            // let hwnd1 = get_by_title("Monitor Settings", None)
            //     .unwrap()
            //     .last()
            //     .unwrap()
            //     .to_owned();

            let hwnd1 = unsafe {GetDesktopWindow()};

            let p = get_by_title(&title, Some(hwnd1)).unwrap().last().unwrap().to_owned();
            let b = get_by_title("OK", Some(p)).unwrap().last().unwrap().to_owned();

            let s = send_message(b, BM_CLICK , 0, 0, Some(5));

            let r = match s {
                Ok(_) => {
                    Ok(DLGID::OK)
                }, 
                Err(e) => {
                    log!(format!("Could not close MsgBox!\nTitle: {}\nText: {}\nHandle: {:?}\nError: {}", title, message, p, e), "e" );
                    Err(HRESULT::E_INVALIDARG)
                }
            };

            r
        }
    };

    let _x = y.join();
    
    return f;
    
}

fn ss_get(key_name: &'static str) -> String{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop = hkcu.open_subkey("Control Panel\\Desktop").unwrap();
    let screen_s: String = desktop.get_value(&key_name).unwrap();

    return screen_s;
}

fn ss_set(key_name: &'static str, key_value: &str){
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Control Panel").join("Desktop");
    let key = hkcu.create_subkey(&path).unwrap().0;

    return key.set_value(&key_name, &key_value).unwrap();
}


// Change Signal RGB
fn change_signal_rgb(profile: &String) -> String{
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

// Change OpenRGB
fn change_open_rgb(addy: &String, port: &String, profile: &String) -> Result<String, String> {
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

// Change VoiceAttack
fn change_voice_attack(profile: &String) -> String {
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

fn sleep(milliseconds: u64){
    let mills = std::time::Duration::from_millis(milliseconds);
    std::thread::sleep(mills);
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

    write_key(&"defaults".to_string(), "gameon", "False");
    write_key(&"General".to_string(), "running", "True");
    write_key(&"General".to_string(), "running_pid", "0");
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
                                write_key(&section_name, "current_priority", "0");
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
            write_key(&section_name, "priority", "0");
        
            section_name = "Idle".to_string();
            write_key(&section_name, "exeName", "300");
            write_key(&section_name, "gameWindowName", "2100-0600");
            write_key(&section_name, "game-or-win", "Game");
            write_key(&section_name, "priority", "4");
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

fn get_ahk_pid(sec: &String) -> Result<u32, String> {
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

fn get_defaults() -> Defaults {
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

fn get_value(section: String, key: String) -> String{
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(section);
    let gamemon = hklm.open_subkey(&path).unwrap();
    gamemon.get_value(key).unwrap()
}

fn write_key(sec_name: &String, key_name: &'static str, key_value: &str){
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("GameMon").join(&sec_name);
    let key = hklm.create_subkey(&path).unwrap().0;

    return key.set_value(&key_name, &key_value).unwrap();
}

fn write_section(sec_name: &String){
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

fn dark_hours(time_range: &String) -> bool {
    let time_of_day = Local::now().time();
    let time_vec = time_range.split("-").collect::<Vec<&str>>();
    let end_num = &time_vec[1].parse::<u64>().unwrap();
    let start_time = NaiveTime::parse_from_str(&time_vec[0], "%H%M").unwrap();
    let end_time = NaiveTime::parse_from_str(&time_vec[1], "%H%M").unwrap();

    if end_num < &1200 {
        if (time_of_day > end_time) && (time_of_day < start_time) {
            return false
        } else {
            return true
        }
    } else {
        if (time_of_day > start_time) && (time_of_day < end_time) {
            return true
        } else {
            return false
        }
    }
}

fn run_cmd(cmd: &String) -> Result<std::process::Child, std::io::Error>{
    let output = Command::new("cmd.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .arg("/c")
        .arg(&cmd)
        .spawn();
    
    return output
}

fn close_pid(pid: u32) -> Result<std::process::Child, std::io::Error>{
    let kill_cmd = format!("TASKKILL /PID {}", &pid);
    let output = Command::new("cmd.exe")
    .creation_flags(CREATE_NO_WINDOW)
    .arg("/c")
    .arg(&kill_cmd)
    .spawn();

return output
}

fn defaults_gui(){
    let my = DefaultsWindow::new(); // instantiate our defaults window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}

fn main_gui(){
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[cfg(windows)]
fn main() {
    // Initialize Setup
    reg_check();
    initialize_log();
    let _cleanup = Cleanup;

    let last_error = std::io::Error::last_os_error().to_string();
    
    if last_error.contains("GameMon"){
        log!(format!("Last shutdown reason: CRASH"), "e");
    } else {
        log!(format!("Last shutdown reason: {}", get_value("defaults".to_string(), "exit_reason".to_string()).to_string()), "w");
    }

    // Create system tray
    let mut tray = TrayItem::new("GameMon", "my-icon-name").unwrap();

    tray.add_label("GameMon").unwrap();
    
    tray.add_menu_item("About", || {
        let hwnd = HWND::GetDesktopWindow();
        hwnd.MessageBox(&format!("GameMon Game Monitor\nBy Akinus21 2022\nWritten in Rust Programming Language").to_string()
        , "About", 
        MB::OK | MB::ICONINFORMATION).unwrap();
    })
    .unwrap();
    
    let (tx, rx) = mpsc::channel();
    let txc = tx.clone();
    
    tray.add_menu_item("View Logs", move || {
        println!("Logs");
        txc.send(Message::Logs).unwrap();
    }).unwrap();

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
    
    loop {

        let mut section;
        let defaults;
        let mem;
        let hklm;
        let path;
        let game_mon;
        let mut time_range;
        let mut ss;
        let mut cmds;
        let mut ahk_run;
        let mut ss_exe;
        let mut ahk_pid;
        let mut ahk_pid_u32;
        let mut ahk_close;
        let mut game_bool;
        let mut win_flag;
        let mut active_pid;
        let mut active_win;

        mem = System::new_all().processes_by_exact_name("GameMon.exe").last().unwrap().memory();

        match mem.cmp(&"1073741824".parse::<u64>().unwrap()){
            Ordering::Greater => {
                exit_app!(0, "Memory allocation too high");
            },
            _ => ()
        }

        match rx.try_recv(){
            Ok(Message::Quit) => exit_app!(1, "Menu"),
            Ok(Message::Gui) => {
                std::thread::spawn(|| {
                    main_gui();
                });
            },
            Ok(Message::Defaults) => {
                std::thread::spawn(||{
                    defaults_gui();
                });
            },
            Ok(Message::Logs) => {
                let _z = run_cmd(&"eventvwr.msc".to_string()).unwrap();
            },
            _ => ()
        };

        // Game and Window Reactions
        defaults = get_defaults();

        hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        path = Path::new("Software").join("GameMon");
        game_mon = hklm.open_subkey(path).unwrap();
        
        for sec in game_mon.enum_keys().map(|x| x.unwrap()){

            

            section = match &sec.as_str() {
                &"defaults" => continue,
                &"General" => continue,
                _ => get_section(&sec),
            };
            
            match &defaults.current_priority.parse::<u64>().unwrap().cmp(&section.priority.parse::<u64>().unwrap()) {
                Ordering::Greater => continue, // DO NOTHING...section is lower priority than current priority
                _ => ()
            };
            
            match &sec.as_str() {
                &"Idle" => { // Begin Idle Reaction
                    write_key(&sec, "exe_name", &ss_get("ScreenSaveTimeOut"));
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
                        false => Err(format!("GetLastInputInfo failed"))
                    }.unwrap().as_secs();
                    time_range = section.game_window_name;

                    match idle_seconds.cmp(&(&section.exe_name.parse::<u64>().unwrap() - 5)) {
                        Ordering::Greater => { // PAST IDLE TIME!!!!!!
                    
                            match &section.running.as_str() {
                                &"False" => { //IDLE IS NOT RUNNING
                                    
                                    
                                    match &defaults.gameon.as_str(){
                                        &"True" => continue,
                                        _ => ()
                                    };

                                    //change values
                                    write_key(&"defaults".to_string(), "current_priority", &section.priority);
                                    write_key(&sec, "running", "True");
                                    write_key(&sec, "running_pid", "0");
                                    write_key(&"General".to_string(), "running", "False");

                                    if dark_hours(&time_range) == true { // WITHIN DARK HOURS!!!!!
                                        log!(&format!("Idle detected! Within dark hours!\nDark Hours are between {}", &time_range));

                                        //change profiles
                                        log!(change_signal_rgb(&defaults.night_hour_srgb_profile));
                                        log!(change_open_rgb(&defaults.orgb_address,
                                            &defaults.orgb_port,
                                            &defaults.night_hour_orgb_profile).unwrap());

                                        // Run other commands                                        
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

                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                for c in s.split(" && ") {
                                                    let _f = match run_cmd(&c.to_string()) {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    };
                                                }
                                            }
                                        }

                                    } else { // NOT WITHIN DARK HOURS!!!!
                                        log!(&format!("Idle detected! Within day hours!\nDay hours are hours outside of the range {}", &time_range));

                                        //change values
                                        write_key(&"General".to_string(), "running", "False");
                                        write_key(&"defaults".to_string(),
                                            "current_priority", &section.priority);

                                        //Make sure display is on
                                        log!("Ensuring Display is ON.");
                                        reg_write_value(&Path::new("Software").join("GameMon"),
                                            "display".to_string(), "on".to_string()).unwrap();

                                        // Run Screensaver
                                        ss_exe = ss_get("SCRNSAVE.EXE");
                                        match get_pid(Some(&ss_exe)) { // Check for Screensaver
                                            Ok(_) => {
                                                match section.game_or_win.as_str() {
                                                    "Yes" => {
                                                        
                                                        let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                        let r = run_cmd(cmd);
                                                        log!(format!("Taking ownership of screensaver...\n\n{:?}", &r));
                                                        
                                                        //change profiles
                                                        log!(change_signal_rgb(&defaults.screensaver_srgb_profile));
                                                        log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.screensaver_orgb_profile).unwrap());
                                                        
                                                        log!("Running Screensaver...");
                                                        ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                        let _z = match run_cmd(&ss) {
                                                            Ok(_) => String::from("OK"),
                                                            Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                        };
                                                    },
                                                    _ => {
                                                        let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                        let _r = run_cmd(cmd);
                                                        //change profiles
                                                        match section.running_pid.as_str() {
                                                            "0" => {
                                                                log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                log!(change_open_rgb(&defaults.orgb_address,
                                                                    &defaults.orgb_port,
                                                                    &section.open_rgbprofile).unwrap());
                                                                write_key(&sec, "running_pid", "1");
                                                            },
                                                            _ => (),
                                                        }
                                                        
                                                    },
                                                }
                                                
                                            },
                                            _ => {
                                                // Run Screensaver
                                                match section.game_or_win.as_str() {
                                                    "Yes" => {
                                                        log!(&format!("Idle is running but screensaver not detected!"));
                                                        //change profiles
                                                        log!("Running Screensaver...");
                                                        log!(change_signal_rgb(&defaults.screensaver_srgb_profile));
                                                        log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.screensaver_orgb_profile).unwrap());
                                                        
                                                        ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                        let _z = match run_cmd(&ss) {
                                                            Ok(_) => String::from("OK"),
                                                            Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                        };
                                                    },
                                                    _ => {
                                                        //change profiles
                                                        match section.running_pid.as_str() {
                                                            "0" => {
                                                                log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                log!(change_open_rgb(&defaults.orgb_address,
                                                                    &defaults.orgb_port,
                                                                    &section.open_rgbprofile).unwrap());
                                                                write_key(&sec, "running_pid", "1");
                                                            },
                                                            _ => (),
                                                        }
                                                    },
                                                }
                                            }
                                        };

                                        // Run other commands
                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                cmds = s.split(" && ");
                                                for c in cmds {
                                                    let _f = match run_cmd(&c.to_string()) {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    };
                                                }
                                            }
                                        }
                                        
                                    };
                                    ahk_run = run_cmd(&section.path_toahk);
                                    assert!(ahk_run.is_ok());
                                    log!(&format!("{} is running!", section.name_ofahk));

                                },
                                _ => { // Idle is running!
                                    
                                    if dark_hours(&time_range) == true { // WITHIN DARK HOURS!!!!!                                       
                                        let _p = match d_quote!(game_mon.get_raw_value("display").unwrap().to_string().as_str()).as_str() {
                                            "on" => { // Display is on past dark hours
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
                                                //change profiles
                                                log!(change_signal_rgb(&defaults.night_hour_srgb_profile));
                                                log!(change_open_rgb(&defaults.orgb_address,
                                                    &defaults.orgb_port,
                                                    &defaults.night_hour_orgb_profile).unwrap());
                                            },
                                            _ => ()
                                        };
                                    } else { // Not WITHIN dark hours!    
                                        ss_exe = ss_get("SCRNSAVE.EXE");                                    
                                        let _p = match d_quote!(game_mon.get_raw_value("display").unwrap().to_string().as_str()).as_str() {
                                            "off" => { // Display is off during day hours                    
                                                log!(&format!("Idle is running; Day Hours Activated.  Turning on Display"));
                                                //Turn on display
                                                let mouse = Mouse::new();
                                                mouse.move_to(0, 0).expect("Failed to turn on monitor(s)!!");
                                                mouse.scroll(5).expect("Failed to scroll wheel!");
                                                reg_write_value(&Path::new("Software").join("GameMon"),
                                                    "display".to_string(), "on".to_string()).unwrap();
                                                
                                                match section.game_or_win.as_str() {
                                                    "Yes" => {
                                                        //change profiles
                                                        match section.running_pid.as_str() {
                                                            "0" => {
                                                                log!(change_signal_rgb(&defaults.screensaver_srgb_profile));
                                                                log!(change_open_rgb(&defaults.orgb_address,
                                                                     &defaults.orgb_port, &defaults.screensaver_orgb_profile).unwrap());
                                                                write_key(&sec, "running_pid", "1");
                                                            },
                                                            _ => (),
                                                        }
                                                        
                                                    },
                                                    _ => {
                                                        let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                        let _r = run_cmd(cmd);
                                                        //change profiles
                                                        match section.running_pid.as_str() {
                                                            "0" => {
                                                                log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                log!(change_open_rgb(&defaults.orgb_address,
                                                                    &defaults.orgb_port,
                                                                    &section.open_rgbprofile).unwrap());
                                                                write_key(&sec, "running_pid", "1");
                                                            },
                                                            _ => (),
                                                        }
                                                        
                                                    },
                                                }
                                            },
                                            _ => {                            
                                                match get_pid(Some(&ss_exe)) { // Check for Screensaver
                                                    Ok(_) => {
                                                        match section.game_or_win.as_str() {
                                                            "Yes" => {
                                                                
                                                                //change profiles
                                                                match section.running_pid.as_str() {
                                                                    "0" => {
                                                                        let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                                        let _r = run_cmd(cmd);

                                                                        ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                                        let _z = match run_cmd(&ss) {
                                                                            Ok(_) => String::from("OK"),
                                                                            Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                                        };
                                                                        log!(change_signal_rgb(&defaults.screensaver_srgb_profile));
                                                                        log!(change_open_rgb(&defaults.orgb_address,
                                                                             &defaults.orgb_port, &defaults.screensaver_orgb_profile).unwrap());
                                                                        write_key(&sec, "running_pid", "1");
                                                                    },
                                                                    _ => (),
                                                                }
                                                            },
                                                            _ => {
                                                                let cmd = &format!("TASKKILL /im /f {}", ss_exe);
                                                                let _r = run_cmd(cmd);
                                                                //change profiles
                                                                match section.running_pid.as_str() {
                                                                    "0" => {
                                                                        log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                        log!(change_open_rgb(&defaults.orgb_address,
                                                                            &defaults.orgb_port,
                                                                            &section.open_rgbprofile).unwrap());
                                                                        write_key(&sec, "running_pid", "1");
                                                                    },
                                                                    _ => (),
                                                                }
                                                                
                                                            },
                                                        }
                                                    },
                                                    _ => {
                                                        // Run Screensaver
                                                        match section.game_or_win.as_str() {
                                                            "Yes" => {
                                                                log!(&format!("Idle is running but screensaver not detected!"));
                                                                //change profiles
                                                                log!(change_signal_rgb(&defaults.screensaver_srgb_profile));
                                                                log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &defaults.screensaver_orgb_profile).unwrap());
                                                                
                                                                log!("Running Screensaver...");
                                                                ss = format!("{} /S", ss_get("SCRNSAVE.EXE"));
                                                                let _z = match run_cmd(&ss) {
                                                                    Ok(_) => String::from("OK"),
                                                                    Err(e) => log!(format!("Failed to run Screensaver!! Command: {} || Error: {}", &ss, &e), "e")
                                                                };
                                                            },
                                                            _ => {
                                                                //change profiles
                                                                match section.running_pid.as_str() {
                                                                    "0" => {
                                                                        log!(change_signal_rgb(&section.signal_rgbprofile));
                                                                        log!(change_open_rgb(&defaults.orgb_address,
                                                                            &defaults.orgb_port,
                                                                            &section.open_rgbprofile).unwrap());
                                                                        write_key(&sec, "running_pid", "1");
                                                                    },
                                                                    _ => (),
                                                                }
                                                                
                                                            },
                                                        }
                                                    }
                                                };
                                            }
                                        };
                                        
                                    };
                                }
                            }
                        },
                        _ => { // NOT PAST IDLE TIME
                            reg_write_value(&Path::new("Software").join("GameMon"),
                                "display".to_string(), "on".to_string()).unwrap();

                            match &section.running.as_str() {
                                &"True" => { //IDLE IS RUNNING
                                    log!(&format!("Idle no longer detected!"));

                                    //change values
                                    write_key(&"General".to_string(), "running", "True");
                                    write_key(&sec, "running", "False");
                                    write_key(&sec, "running_pid", "0");
                                    write_key(&"defaults".to_string(), "current_priority", "0");

                                    //run extra commands
                                    ahk_pid = get_ahk_pid(&sec.to_string());
                                    assert!(ahk_pid.is_ok());
                                    ahk_pid_u32 = ahk_pid.unwrap();

                                    ahk_close = close_pid(ahk_pid_u32);
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

                    match section.game_or_win.as_str() {
                        "Game" => {
                            
                            //is program running?
                            
                            game_bool = get_pid(Some(&section.exe_name));
                            match game_bool {
                                Ok(0) => { //Program not found
                                    match &section.running.as_str() {
                                        &"True" => { //Profile is on
                                            log!(&format!("{} no longer detected!", &section.exe_name));

                                            //change values
                                            write_key(&"General".to_string(), "running", "True");
                                            write_key(&sec, "running", "False");
                                            write_key(&"defaults".to_string(), "gameon", "False");
                                            write_key(&"defaults".to_string(), "current_priority", "0");

                                            //run extra commands
                                            ahk_pid = get_ahk_pid(&sec.to_string());
                                            assert!(ahk_pid.is_ok());
                                            ahk_pid_u32 = ahk_pid.unwrap();

                                            ahk_close = close_pid(ahk_pid_u32);
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
                                            write_key(&"General".to_string(), "running", "False");
                                            write_key(&"defaults".to_string(), "gameon", "True");
                                            write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                            //change profiles
                                            log!(change_signal_rgb(&section.signal_rgbprofile));
                                            log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).unwrap());
                                            
                                            match &section.voice_attack_profile.as_str() {
                                                &"" => log!("No VoiceAttack profile found."),
                                                _ => log!(&change_voice_attack(&section.voice_attack_profile))
                                            };

                                            //run extra commands
                                            ahk_run = run_cmd(&section.path_toahk);
                                            assert!(ahk_run.is_ok());
                                            log!(&format!("{} is running!", &section.name_ofahk));
                                            match &section.other_commands.as_str() {
                                                &"" => (),
                                                s => {
                                                    cmds = s.split(" && ");
                                                    for c in cmds {
                                                        let _f = match run_cmd(&c.to_string()) {
                                                            Ok(_) => log!(format!("Running {}", &c)),
                                                            Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                        };
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

                            match &get_value("Idle".to_string(), "running".to_owned()).as_str() {
                                &"True" => continue,
                                _ => ()
                            }

                            win_flag = &defaults.window_flag;

                            // is window active?
                            active_pid = get_active_window().unwrap().process_id;
                            active_win = name_by_pid(Pid::from(active_pid as usize)).unwrap();

                            if &active_win == &section.exe_name{ // ** WINDOW IS ACTIVE **
                                match &section.running.as_str() {
                                    &"False" => { //Profile Off
                                        log!(&format!("{} detected!", &section.exe_name));
                                        
                                        //change values
                                        write_key(&sec, "running", "True");
                                        write_key(&sec, "running_pid", &active_pid.to_string());
                                        write_key(&"General".to_string(), "running", "False");
                                        write_key(&"defaults".to_string(), "window_flag", &sec);
                                        write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                        //change profiles
                                        log!(change_signal_rgb(&section.signal_rgbprofile));
                                        log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).unwrap());
                                        
                                        match &section.voice_attack_profile.as_str() {
                                            &"" => log!("No VoiceAttack profile found."),
                                            _ => log!(&change_voice_attack(&section.voice_attack_profile))
                                        };

                                        //run extra commands
                                        ahk_run = run_cmd(&section.path_toahk);
                                        assert!(ahk_run.is_ok());
                                        log!(&format!("{} is running!", &section.name_ofahk));
                                        match &section.other_commands.as_str() {
                                            &"" => (),
                                            s => {
                                                cmds = s.split(" && ");
                                                for c in cmds {
                                                    let _f = match run_cmd(&c.to_string()) {
                                                        Ok(_) => log!(format!("Running {}", &c)),
                                                        Err(e) => log!(format!("Could not run {}: {}", &c, &e), "e"),
                                                    };
                                                }
                                            }
                                        }
                                        
                                        
                                    },
                                    _ => {
                                        if &win_flag == &&sec {
                                            continue;
                                        } else {
                                            write_key(&"defaults".to_string(), "window_flag", &sec);
                                            write_key(&"General".to_string(), "running", "False");
                                            write_key(&"defaults".to_string(), "current_priority", &section.priority);

                                            //change profiles
                                            log!(change_signal_rgb(&section.signal_rgbprofile));
                                            log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).unwrap());
                                            
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
                                            write_key(&"defaults".to_string(), "window_flag", "General");
                                            write_key(&"General".to_string(), "running", "True");
                                            write_key(&"defaults".to_string(), "current_priority", "0");

                                        }

                                        //run extra commands
                                        ahk_pid = get_ahk_pid(&sec.to_string());
                                        assert!(ahk_pid.is_ok());
                                        ahk_pid_u32 = ahk_pid.unwrap();

                                        ahk_close = close_pid(ahk_pid_u32);
                                        assert!(ahk_close.is_ok());
                                        log!(&format!("{} is no longer running!", section.name_ofahk));

                                    },
                                    _ => {
                                        if &win_flag == &&sec {
                                            write_key(&"defaults".to_string(), "window_flag", "General");                                            
                                        }
                                    }
                                };
                            }
                        }
                        _ => ()
                    };
                } //End Game or Window Reaction
                
            }; // End of section match
            sleep(100);

        }; // End of section "For" loop
        section = get_section(&"General".to_string());
        if &section.running == "True" {
            if &section.running_pid == "0" {
                //change profiles
                log!(change_signal_rgb(&section.signal_rgbprofile));
                log!(change_open_rgb(&defaults.orgb_address, &defaults.orgb_port, &section.open_rgbprofile).unwrap());
                
                match &section.voice_attack_profile.as_str() {
                    &"" => log!("No VoiceAttack profile found."),
                    _ => log!(&change_voice_attack(&section.voice_attack_profile))
                };
            };
                
                write_key(&"General".to_string(), "running_pid", "1");
                write_key(&"defaults".to_string(), "current_priority", "0");
        } else {
            match section.running_pid.as_str() {
                "0" => (),
                _ => {
                    write_key(&"General".to_string(), "running_pid", "0");
                }

            };
        };

        
    };
    
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
