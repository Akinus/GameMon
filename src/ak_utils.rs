// Rust Programming Language
// #####################################################################
// File: ak_utils.rs                                                   #
// Project: src                                                        #
// Created Date: Sat, 10 Dec 2022 @ 12:41:23                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sat, 10 Dec 2022 @ 14:13:34                          #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

use chrono::{Local, NaiveTime};

use crate::ak_utils::macros::exit_app;
use crate::ak_run::close_all_ahk;

//   Import Data ####
pub fn sleep(milliseconds: u64){
    let mills = std::time::Duration::from_millis(milliseconds);
    std::thread::sleep(mills);
}

pub fn dark_hours(time_range: &String) -> bool {
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

pub fn url_encode(data: String) -> String{
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
            log::info!("{}", $a);
            $a
        }
    };

    ($a:expr,$b:expr) => {
        {
            match $b {
                "i" => {
                    log::info!("{}", $a);
                    $a
                },
                "d" => {
                    log::debug!("{}", $a);
                    $a                   
                },
                "e" => {
                    log::error!("{}", $a);
                    $a                    
                },
                "w" => {
                    log::warn!("{}", $a);
                    $a                    
                },
                "t" => {
                    log::trace!("{}", $a);
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

    pub(crate) use d_quote;
    pub(crate) use exit_app;
    pub(crate) use log;

}