use std::process::{Command, Stdio};

use hyprland::{data::Client, shared::HyprDataActiveOptional};

fn main() -> hyprland::shared::HResult<()> {
    let Some(active_window) = Client::get_active()? else {
        Command::new("alacritty").spawn().unwrap(); return Ok(())
    };
    if active_window.title.contains("Zellij") {
        let session = active_window.title.split_at(8).1.split(')').next().unwrap();
        println!("{session}");
        Command::new("zellij")
            .arg("-s")
            .arg(session)
            .arg("action")
            .arg("new-pane")
            .spawn()
            .unwrap();
    } else if active_window.class.contains("Alacritty") {
        fun_name(active_window);
    } else {
        Command::new("alacritty").spawn().unwrap();
    };
    Ok(())
}

fn fun_name(active_window: Client) {
    println!("{}", active_window.title);
    // println!("{cwd}");
    let pgrep = Command::new("pgrep")
        .arg("-P")
        .arg(format!("{}", active_window.pid))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let pgrep_output = pgrep.wait_with_output().unwrap().stdout;
    let child = pgrep_output
        .split(|byte| *byte == '\n' as u8)
        .next()
        .unwrap();
    let child_pid = child.iter().map(|byte| *byte as char).collect();
    println!("{child_pid}");
    let cwd: String = Command::new("pwdx")
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
        .collect();
    Command::new("alacritty")
        .arg("--working-directory")
        .arg(cwd)
        .spawn()
        .unwrap();
}
