use std::collections::HashMap;
use std::env::args;
use std::process::{Command, Stdio};

use hyprland::{data::Client, shared::HyprDataActiveOptional};

fn main() {
    if let None = sub_main() {
        launch_terminal("kitty", "--directory", "~", "-e", "")
    }
}

fn sub_main() -> Option<()> {
    let other_args = args().into_iter().skip(1);
    if let Some(active_window) = Client::get_active()? {
        if let Some((command, dir_flag, skip, exec_flag)) =
            get_termainals().get(&active_window.class)
        {
            let pid = get_child_pid(active_window, *skip);
            let cwd = get_child_cwd(&pid);
            launch_terminal(command, dir_flag, &cwd, exec_flag, other_args)
        } else {
            launch_terminal("kitty", "--directory", "~", "-e", other_args)
        };
    } else {
        // well
        launch_terminal("kitty", "--directory", "~", "-e", other_args)
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

fn launch_terminal(
    terminal: &str,
    working_directory_flag: &str,
    working_directory: &str,
    exec_flag: &str,
    other_args: impl Iterator<Item = String>,
) {
    let mut cmd = Command::new(terminal);
    cmd.arg(working_directory_flag);
    cmd.arg(working_directory);
    if other_args.size_hint().0 > 0 {
        cmd.arg(exec_flag);
    }

    for arg in other_args {
        cmd.arg(arg);
    }
    cmd.spawn().unwrap();
}

fn get_termainals() -> HashMap<String, (&'static str, &'static str, usize, &'static str)> {
    [
        // (class, (spawn_command, directory_flag, child_procces_index))

        // The cwd is obtained by running pwdx on the shell...
        // Most terminals have the shell as thier only child but kitty has two..
        // The index tells the program which output of pgrep to use, probably 0
        (
            "Alacritty".to_owned(),
            ("alacritty", "--working-directory", 0, "-e"),
        ),
        ("kitty".to_owned(), ("kitty", "--directory", 1, "-e")),
    ]
    .into()
}
