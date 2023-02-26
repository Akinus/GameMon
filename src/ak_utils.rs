// Rust Programming Language
// #####################################################################
// File: ak_utils.rs                                                   #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:41:23                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Wed, 22 Feb 2023 @ 22:07:53                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

use std::path::{Path, PathBuf};

use chrono::NaiveTime;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use crate::ak_io::read::get_value;
use crate::ak_io::write::reg_write_value;
use crate::ak_run::close_all_ahk;
use crate::ak_utils::macros::exit_app;

//   Import Data ####
pub fn sleep(milliseconds: u64) {
    let mills = std::time::Duration::from_millis(milliseconds);
    std::thread::sleep(mills);
}

pub fn dark_hours() -> bool {
    let time_range = get_value(HKEY, "Idle", "game_window_name");
    let start_hour = time_range[0..2].parse().unwrap();
    let start_minute = time_range[2..4].parse().unwrap();
    let end_hour = time_range[5..7].parse::<u32>().unwrap();
    let end_minute = time_range[7..9].parse().unwrap();

    let start_time = NaiveTime::from_hms_opt(start_hour, start_minute, 0);
    let end_time = NaiveTime::from_hms_opt(end_hour, end_minute, 0);

    // Get the current time
    let current_time = chrono::Local::now().time();

    // Return true if the current hour and minute are within the specified range (inclusive)
    if (start_hour > 12)
        && (end_hour < 12)
        && ((current_time < start_time.unwrap()) && (current_time > end_time.unwrap()))
    {
        let _v = reg_write_value(
            &RegKey::predef(HKEY_LOCAL_MACHINE),
            &Path::new("Software").join("GameMon"),
            "night".to_string(),
            "false".to_string(),
        );
        return false;
    } else {
        let _v = reg_write_value(
            &RegKey::predef(HKEY_LOCAL_MACHINE),
            &Path::new("Software").join("GameMon"),
            "night".to_string(),
            "true".to_string(),
        );
        return true;
    }
}

pub fn url_encode(data: String) -> String {
    let data = str::replace(&data, "\n", "%0A");
    let data = str::replace(&data, "+", "%2b");
    let data = str::replace(&data, "\r", "%0D");
    let data = str::replace(&data, "'", "%27");
    let data = str::replace(&data, " ", "%20");
    let data = str::replace(&data, "#", "%23");
    let data = str::replace(&data, "&", "^&");
    return data;
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Gui,
    Defaults,
    Logs,
}

pub const HKEY: &winreg::RegKey = &RegKey::predef(HKEY_LOCAL_MACHINE);

pub struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        exit_app!();
    }
}

pub mod macros {

    macro_rules! d_quote {
        ($a:expr) => {
            quoted_string::strip_dquotes($a).unwrap().to_string()
        };
    }

    macro_rules! exit_app {
        ($a:expr) => {{
            let mut log_text = format!("Exiting.  Reason: Shutdown\n");
            log_text.push_str(format!("{}\n", reset_running()));

            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log_text.push_str(format!("All ahk scripts are closed"));

            let mut path = Path::new("Software").join("GameMon");
            reg_write_value(HKEY, &path, "exit_reason", "Shutdown");

            eventlog::deregister("GameMon Log").unwrap();

            log!(log_text, "w");

            std::process::exit($a);
        }};

        ($b:expr) => {{
            let mut log_text = format!("Exiting. Reason: {}\n", $b);
            log_text.push_str(format!("{}\n", reset_running()));
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log_text.push_str(format!("All ahk scripts are closed\n"));
            let mut path = Path::new("Software").join("GameMon");
            reg_write_value(HKEY, &path, "exit_reason", $b);
            eventlog::deregister("GameMon Log").unwrap();
            log!(log_text, "w");
            std::process::abort();
        }};

        ($c:expr) => {{
            let mut log_text = format!("Exiting.  Reason: Shutdown\n");
            for handle in $c {
                handle.join().unwrap();
            }
            log_text.push_str(format!("{}\n", reset_running()));
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log_text.push_str(format!("All ahk scripts are closed\n"));
            let mut path = Path::new("Software").join("GameMon");
            reg_write_value(HKEY, &path, "exit_reason", "Shutdown");
            eventlog::deregister("GameMon Log").unwrap();
            log!(log_text, "w");
            std::process::exit(0);
        }};

        ($a:expr,$b:expr) => {{
            let mut log_text = format!("Exiting. Reason: {}\n", $b);
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log_text.push_str(&format!("All ahk scripts are closed\n"));

            let path = std::path::Path::new("Software").join("GameMon");
            crate::ak_io::write::reg_write_value(HKEY, &path, "exit_reason", $b).unwrap();

            eventlog::deregister("GameMon Log").unwrap();
            log!(log_text, "w");
            std::process::exit($a);
        }};

        ($a:expr,$b:expr, $c:expr) => {{
            let mut log_text = format!("Exiting. Reason: {}\n", $b);
            for handle in $c {
                handle.join().unwrap();
            }
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            log_text.push_str(format!("All ahk scripts are closed\n"));

            let mut path = Path::new("Software").join("GameMon");
            reg_write_value(HKEY, &path, "exit_reason", $b);

            eventlog::deregister("GameMon Log").unwrap();
            log!(log_text, "w");
            std::process::exit($a);
        }};

        () => {{
            let all_close = close_all_ahk();
            assert!(all_close.is_ok());
            eventlog::deregister("GameMon Log").unwrap();
            std::process::abort();
        }};
    }

    macro_rules! log {
        ($a:expr) => {{
            log::info!("{}", $a);
            $a
        }};

        ($a:expr,$b:expr) => {{
            match $b {
                "i" => {
                    log::info!("{}", $a);
                    $a
                }
                "d" => {
                    log::debug!("{}", $a);
                    $a
                }
                "e" => {
                    log::error!("{}", $a);
                    $a
                }
                "w" => {
                    log::warn!("{}", $a);
                    $a
                }
                "t" => {
                    log::trace!("{}", $a);
                    $a
                }
                _ => $a,
            }
        }};

        () => {{
            trace!("{}", "BREAK BREAK BREAK ----------------");
            $a
        }};
    }

    pub(crate) use d_quote;
    pub(crate) use exit_app;
    pub(crate) use log;
}
