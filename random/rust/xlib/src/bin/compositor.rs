use std::{
    process::Command,
    ptr,
};

use x11::{
    xlib,
    xcomposite
};

fn main() {
    run_demo_app();
}

fn run_demo_app() {
    Command::new("xterm")
        .output()
        .expect("Failed to launch xterm.");
}