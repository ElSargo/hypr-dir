use std::collections::HashMap;
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
    } else if let Some((command, dir_flag, skip)) = get_termainals().get(&active_window.class) {
        let pid = get_child_pid(active_window, *skip);
        let cwd = get_child_cwd(&pid);
        launch_terminal(command, dir_flag, &cwd)
    } else {
        Command::new("alacritty").spawn().unwrap();
    };
    Ok(())
}

fn get_child_pid(active_window: Client, skip_childern: usize) -> Vec<u8> {
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
        .skip(skip_childern)
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

fn launch_terminal(terminal: &str, working_directory_arg: &str, working_directory: &str) {
    Command::new(terminal)
        .arg(working_directory_arg)
        .arg(working_directory)
        .spawn()
        .unwrap();
}

fn get_termainals() -> HashMap<String, (&'static str, &'static str, usize)> {
    [
        // (class, (spawn_command, directory_flag, child_procces_index))

        // The cwd is obtained by running pwdx on the shell...
        // Most terminals have the shell as thier only child but kitty has two..
        // The index tells the program which output of pgrep to use, probably 0
        (
            "Alacritty".to_owned(),
            ("alacritty", "--working-directory", 0),
        ),
        ("kitty".to_owned(), ("kitty", "--directory", 1)),
        ("st-256color".to_owned(), ("st", "-d", 0)),
    ]
    .into()
}
