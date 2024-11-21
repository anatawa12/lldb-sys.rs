use cc::Build;

use std::{fs, process::Command};

fn get_compiler_config() -> Build {
    println!("cargo:rerun-if-env-changed=LLDB_INCLUDE_DIRS");
    let mut res = cc::Build::new();
    res.try_flags_from_environment(concat!(env!("CARGO_PKG_NAME"), "_CFLAGS")).ok();
    if let Some(dirs) = std::env::var_os("LLDB_INCLUDE_DIRS") {
        for path in std::env::split_paths(&dirs) {
            res.include(path);
        }
    }
    res
}

fn main() {
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    get_compiler_config()
        .cpp(true)
        .flag("-std=c++14")
        .warnings(false)
        .include("src")
        .file("src/lldb/UnityBuild.cpp")
        .compile("liblldb-c.a");
}
