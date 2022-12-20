// Rust Programming Language
// #####################################################################
// File: ak_gui.rs                                                     #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:54:42                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Mon, 19 Dec 2022 @ 22:20:16                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
pub mod windows {
    use std::{path::Path, fs::{File, OpenOptions}, cmp::Ordering};
    use std::io::Write;

    use msgbox;
    use native_dialog::FileDialog;
    use windows_win::{raw::window::{
        get_by_title,
        send_message
    }};
    use winapi::{
        um::{
            winuser::{
                BM_CLICK, GetDesktopWindow
            },
        }};
    use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};
    use winsafe::{co::{DLGID, HRESULT, COLOR, WS, SS, WS_EX}, gui, WString, SIZE, POINT, prelude::{GuiWindowText, GuiNativeControlEvents, GuiWindow, GuiParent, GuiEvents, user_Hwnd, GuiChildFocus}};
    
    use crate::{ak_utils::{sleep, macros::{log}}, ak_io::{write::{write_key, ss_set, reset_running, write_section, delete_section}, read::{get_section, ss_get, get_defaults, get_value}}};
    
    pub fn msg_box(title: Option<&str>, message: Option<&str>, exit_wait_time_in_ms: u64) -> Result<DLGID, HRESULT> {
        let ftitle = match title {
            Some(t) => t,
            None => "GameMon"
        };
        
        let fmessage = match message {
            Some(m) => m,
            None => "Test"
        };

        let til = String::from(ftitle);
        let msg = String::from(fmessage);

        return match exit_wait_time_in_ms {
            0 => {
                let _rr = msgbox::create(&ftitle, &fmessage, msgbox::IconType::Info);
                Ok(DLGID::OK)
            },
            _ => {
                
                std::thread::spawn(move ||{
                    let _rr = msgbox::create(&til,&msg, msgbox::IconType::Info);
            
                });
                sleep(exit_wait_time_in_ms);
    
    
                let hwnd1 = unsafe {GetDesktopWindow()};
    
                let p = get_by_title(&ftitle, Some(hwnd1)).unwrap().last().unwrap().to_owned();
                let b = get_by_title("OK", Some(p)).unwrap().last().unwrap().to_owned();
    
                let s = send_message(b, BM_CLICK , 0, 0, Some(5));
    
                let r = match s {
                    Ok(_) => {
                        Ok(DLGID::OK)
                    }, 
                    Err(e) => {
                        log!(format!("Could not close MsgBox!\nTitle: {ftitle}\nText: {fmessage}\nHandle: {p:?}\nError: {e}"), "e" );
                        Err(HRESULT::E_INVALIDARG)
                    }
                };
    
                r
            }
        };        
    }

    #[derive(Clone)]
    pub struct DefaultsWindow {
        pub wnd: gui::WindowMain,
        pub label_openRGBpath: gui::Label,
        pub edit_openRGBpath: gui::Edit,
        pub label_voiceAttackPath: gui::Label,
        pub edit_voiceAttackPath: gui::Edit,
        pub label_defaultORGBProfile: gui::Label,
        pub edit_defaultORGBProfile: gui::Edit,
        pub label_defaultSRGBProfile: gui::Label,
        pub edit_defaultSRGBProfile: gui::Edit,
        pub label_screensaver_orgb_profile: gui::Label,
        pub edit_screensaver_orgb_profile: gui::Edit,
        pub label_screensaver_srgb_profile: gui::Label,
        pub edit_screensaver_srgb_profile: gui::Edit,
        pub label_night_hour_orgb_profile: gui::Label,
        pub edit_night_hour_orgb_profile: gui::Edit,
        pub label_night_hour_srgb_profile: gui::Label,
        pub edit_night_hour_srgb_profile: gui::Edit,
        pub label_orgb_port: gui::Label,
        pub edit_orgb_port: gui::Edit,
        pub label_orgb_address: gui::Label,
        pub edit_orgb_address: gui::Edit,
        pub btn_save: gui::Button,
        pub btn_close: gui::Button,
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
                    match msg_box(None, Some("Saving..."), 1000) {
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
        pub wnd: gui::WindowMain,
        pub btn_add: gui::Button,
        pub edit_add: gui::Edit,
        pub btn_delete: gui::Button,
        pub main_list: gui::ListBox,
        pub label_exe: gui::Label,
        pub edit_exe: gui::Edit,
        pub label_win_name: gui::Label,
        pub edit_win_name: gui::Edit,
        pub label_ahk_name: gui::Label,
        pub edit_ahk_name: gui::Edit,
        pub label_ahk_path: gui::Label,
        pub edit_ahk_path: gui::Edit,
        pub label_orgb_profile: gui::Label,
        pub edit_orgb_profile: gui::Edit,
        pub btn_find: gui::Button,
        pub label_srgb_profile: gui::Label,
        pub edit_srgb_profile: gui::Edit,
        pub label_va_profile: gui::Label,
        pub edit_va_profile: gui::Edit,
        pub label_game_win: gui::Label,
        pub btn_save: gui::Button,
        pub radio_game_win: gui::RadioGroup,
        pub label_priority: gui::Label,
        pub radio_priority: gui::RadioGroup,
        pub btn_close: gui::Button,
        pub label_other_commands: gui::Label,
        pub btn_cmd_add: gui::Button,
        pub edit_cmd_add: gui::Edit,
        pub btn_cmd_delete: gui::Button,
        pub cmd_list: gui::ListBox,

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

                    item_vec.sort_by_key(|a| a.to_lowercase());
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
                    
                    match sec.as_str() {
                        "defaults" => (),
                        "Idle" => {
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
                            self2.label_priority.set_text("Priority....");
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
                            self2.label_priority.set_text("Priority....");
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
                        if sec.as_str() == "Idle" {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        };

                        match msg_box(None, Some("Saving..."), 1000) {
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
                        match msg_box(None, Some("Saving..."), 1000) {
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
                    let mut final_string = "".to_owned();
                    let reg_cmds = get_value(sec.clone(), "other_commands".to_string());
                    if reg_cmds.is_empty(){
                        final_string.push_str(&self2.edit_cmd_add.text());
                            
                        write_key(&sec, "other_commands", &final_string);
                    } else {
                        let co = reg_cmds.split(" && ");
                        let mut cmd_vec = Vec::new();
                        for o in co {
                            cmd_vec.push(o);
                        };
                        let len = cmd_vec.len() as u32;
                        if len.cmp(&(1 as u32)) == Ordering::Greater {
                            for c in cmd_vec {
                                final_string.push_str(c);
                                final_string.push_str(" && ");
                            }
                            
                            final_string.push_str(&self2.edit_cmd_add.text());
                            write_key(&sec, "other_commands", &final_string);
                        } else {
                            final_string.push_str(&reg_cmds);
                            final_string.push_str(" && ");
                            final_string.push_str(&self2.edit_cmd_add.text());
                            write_key(&sec, "other_commands", &final_string);
                        }
                        
                    };
                   
                    self2.edit_cmd_add.set_text("");
                    let reg_cmds = get_value(sec.clone(), "other_commands".to_string());
                    self2.cmd_list.items().delete_all();
                    match &reg_cmds.as_str() {
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
                    let mut needle = cmd.to_owned();
                    needle.push_str(" && ");
                    
                    let haystack = &section.other_commands;

                    if haystack.contains(&needle) {
                        let final_string = str::replace(haystack, &needle, "");
                        
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
                        let final_string = str::replace(haystack, &cmd.to_string(), "");
                        
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
                        
                        write_key(&sec, "other_commands", final_string);
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
            
                            item_vec.sort_by_key(|a| a.to_lowercase());
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
                    delete_section(sec);
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

                    item_vec.sort_by_key(|a| a.to_lowercase());
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

                    if sec.as_str() == "Idle"{
                        ss_set("ScreenSaveTimeOut", &self2.edit_exe.text());
                    };

                    write_key(&sec, "game_window_name", &self2.edit_win_name.text());

                    if self2.edit_ahk_path.text().is_empty() {
                        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                        let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
                        let mut script_dir: String = g_key.get_value("InstallDir").unwrap();
                        let script_dirname = "\\scripts";
                        script_dir.push_str(script_dirname);
                        script_dir.push_str(&format!("\\{}.ahk", &sec));

                        File::create(&script_dir).unwrap();
                        let mut lfile = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&script_dir)
                            .unwrap();
                        write!(lfile, "#Persistent\n#SingleInstance, Force\n#NoTrayIcon").unwrap();

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
                        if sec.as_str() == "Idle" {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        };
                    }
                    if let Some(priority) = self2.radio_priority.checked() {
                        let gow = priority.hwnd().GetWindowText()?;
                        write_key(&sec, "priority", &gow);
                    }
                    log!(&format!("Saved settings for {}...", &sec));
                    match msg_box(None, Some("Saving..."), 1000) {
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

                    if sec.as_str() == "Idle" {
                        ss_set("ScreenSaveTimeOut", &self2.edit_exe.text());
                    };

                    write_key(&sec, "game_window_name", &self2.edit_win_name.text());

                    if self2.edit_ahk_path.text().is_empty() {
                        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                        let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
                        let mut script_dir: String = g_key.get_value("InstallDir").unwrap();
                        let script_dirname = "\\scripts";
                        script_dir.push_str(script_dirname);
                        script_dir.push_str(&format!("\\{}.ahk", &sec));

                        File::create(&script_dir).unwrap();
                        let mut lfile = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&script_dir)
                            .unwrap();
                        write!(lfile, "#Persistent\n#SingleInstance, Force\n#NoTrayIcon").unwrap();

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
                        if sec.as_str() == "Idle" {
                            let new_gow = match gow.as_str() {
                                "Yes" => "1",
                                _ => "0"
                            };
                            ss_set("ScreenSaveActive", new_gow);
                        };
                    }
                    if let Some(priority) = self2.radio_priority.checked() {
                        let gow = priority.hwnd().GetWindowText()?;
                        write_key(&sec, "priority", &gow);
                    }
                    log!(&format!("Saved settings for {}...", &sec));
                    match msg_box(None, Some("Saving..."), 1000) {
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

    pub fn defaults_gui(){
        let my = DefaultsWindow::new(); // instantiate our defaults window
        if let Err(e) = my.wnd.run_main(None) { // ... and run it
            eprintln!("{e}");
        }
    }
    
    pub fn main_gui(){
        let my = MyWindow::new(); // instantiate our main window
        if let Err(e) = my.wnd.run_main(None) { // ... and run it
            eprintln!("{e}");
        }
    }
}