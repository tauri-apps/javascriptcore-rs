// Generated by gir (https://github.com/gtk-rs/gir @ e8f82cf)
// from ../gir-files (@ 7d95377)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
