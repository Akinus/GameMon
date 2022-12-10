// Rust Programming Language
// #####################################################################
// File: build.rs                                                      #
// Project: GameMon                                                    #
// Created Date: Sat, 17 Sep 2022 @ 23:47:20                           #
// Author: Akinus21                                                    #
// -----                                                               #
// Last Modified: Sun, 20 Nov 2022 @ 9:17:01                           #
// Modified By: Akinus21                                               #
// HISTORY:                                                            #
// Date      	By	Comments                                           #
// ----------	---	-------------------------------------------------- #
// #####################################################################

//   Import Data ####
use windres::Build;

fn main() {
    Build::new().compile("tray-example.rc").unwrap();
}