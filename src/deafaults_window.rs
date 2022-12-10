// Rust Programming Language
// #####################################################################
// File: deafaults_window.rs                                           #
// Project: src                                                        #
// Created Date: Sun, 16 Oct 2022 @ 16:28:23                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 16 Oct 2022 @ 16:38:25                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
use winsafe::{prelude::*, gui, POINT, SIZE};
use crate::ids;
use ini::Ini;

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
                let path = ini_file();
                let i = Ini::load_from_file(path).unwrap();
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
                self2.wnd.hwnd().DestroyWindow()?;
                Ok(())
            }
        });
    }
}

fn ini_file() -> String{
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let g_key = hklm.open_subkey("SOFTWARE\\GameMon").unwrap();
    let mut ini_file: String = g_key.get_value("InstallDir").unwrap();
    ini_file.push_str("\\gamemon.ini");
    return ini_file;
}