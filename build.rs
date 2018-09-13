/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut cmake = cmake::Config::new(".");
    // Cross-compiling for android needs the target OS set to Linux.
    if target.contains("android") {
        cmake.define("CMAKE_SYSTEM_NAME", "Linux");
    }
    let dst = cmake.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=azure");
        println!("cargo:rustc-link-lib=uuid");
        if target.contains("gnu") {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }
}
