use std::collections::HashMap;
use std::env::args;
use std::process::{Command, Stdio};

use hyprland::{data::Client, shared::HyprDataActiveOptional};

fn main() {
    if let None = sub_main() {
        let iterator = vec!["".to_owned()].into_iter();
        launch_kitty("", iterator);
    }
}

fn sub_main() -> Option<()> {
    let other_args = args().into_iter().skip(1);
    if let Some(active_window) = Client::get_active().ok()? {
        if &active_window.class == "kitty" {
            let pid = get_child_pid(active_window);
            let cwd = get_child_cwd(&pid);
            launch_kitty(&cwd, other_args)
        } else {
            launch_kitty("", other_args)
        };
    } else {
        // well
        launch_kitty("", other_args)
    };
    Some(())
}

fn get_child_pid(active_window: Client) -> Vec<u8> {
    println!("{}", active_window.title);
    // println!("{cwd}");
    let pgrep = Command::new("pgrep")
        .arg("-P")
        .arg(format!("{}", active_window.pid))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let pgrep_output = pgrep.wait_with_output().unwrap().stdout;
    pgrep_output
        .split(|byte| *byte == '\n' as u8)
        .skip(1)
        .next()
        .unwrap()
        .to_vec()
}

fn get_child_cwd(child: &Vec<u8>) -> String {
    let child_pid = child.iter().map(|byte| *byte as char).collect();
    Command::new("pwdx")
        .arg::<String>(child_pid)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout
        .split(|byte| *byte == ' ' as u8)
        .last()
        .unwrap()
        .iter()
        .map(|byte| *byte as char)
        .filter(|c| *c != '\n')
        .collect()
}

fn launch_kitty(working_directory: &str, other_args: impl Iterator<Item = String>) {
    let mut cmd = Command::new("kitty");
    cmd.arg("--single-instance");
    cmd.arg("--directory");
    cmd.arg(working_directory);
    if other_args.size_hint().0 > 0 {
        cmd.arg("-e");
    }

    for arg in other_args {
        cmd.arg(arg);
    }
    cmd.spawn().unwrap();
}
