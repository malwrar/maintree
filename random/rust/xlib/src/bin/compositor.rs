use std::{
    process::Command,
    thread,
};

use xlib_sandbox::compositor::Compositor;

fn main() {
    env_logger::init();

    thread::spawn(|| {
        run_demo_app();
    });

    let mut compositor = Compositor::local().unwrap();
    compositor.run().unwrap();
}

fn run_demo_app() {
    Command::new("xterm")
        .args(&["-hold", "-e", "top", "-d", "0.1"])
        .output()
        .expect("Failed to launch xterm.");
}