// Rust Programming Language
// #####################################################################
// File: my_window.rs                                                  #
// Project: src                                                        #
// Created Date: Sun, 16 Oct 2022 @ 16:25:43                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 16 Oct 2022 @ 16:33:19                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
use winsafe::{prelude::*, gui};
use winsafe::{gui, POINT, SIZE};
use crate::ids;

#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain, // responsible for managing the window
    btn_add: gui::Button,     // a button
    edit_add: gui::Edit,
    btn_delete: gui::Button,     // a button
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

}

impl MyWindow {
    pub fn new() -> Self {
        
        let last_y = 10;

        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "GameMon - Monitor Settings".to_owned(),
                size: SIZE::new(900, 510),
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

        let btn_save = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Save".to_owned(),
                position: POINT::new(800, last_y - 5),
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

        let btn_close = gui::Button::new(
            &wnd, 
            gui::ButtonOpts {
                text: "&Close".to_owned(),
                position: POINT::new(800, last_y - 5),
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
            
        };
        new_self.events(); // attach our events
        new_self
    }

    pub fn run(&self) -> gui::MsgResult<i32> {
		self.wnd.run_main(None)
	}

    fn events(&self) {
        self.wnd.on().wm_create({ // happens once, right after the window is created
			let self2 = self.clone();
			move |_| {
                let path = ini_file();
                let i = Ini::load_from_file(path).unwrap();
                let defaults = get_defaults();
                let mut item_vec = Vec::new();

                for (sect, prop) in i.iter() {
                    let sec = sect.unwrap().to_string();
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

        self.btn_add.on().bn_clicked({
            let self2 = self.clone();
            move || {
                let new = self2.edit_add.text();
                match new.as_str() {
                    "" => (),
                    _ => {
                        self2.edit_add.set_text("");

                        write_section(&new);
                        log("i", &format!("Added monitor {}...", &new));

                        self2.main_list.items().delete_all();
        
                        let path = ini_file();
                        let i = Ini::load_from_file(path).unwrap();
                        let mut item_vec = Vec::new();
        
                        for (sect, prop) in i.iter() {
                            let sec = sect.unwrap().to_string();
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
                log("i", &format!("Deleted monitor {}...", &sec));

                self2.main_list.items().delete_all();

                let path = ini_file();
                let i = Ini::load_from_file(path).unwrap();
                let mut item_vec = Vec::new();

                for (sect, prop) in i.iter() {
                    let sec = sect.unwrap().to_string();
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
                log("i", &format!("Saved settings for {}...", &sec));
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