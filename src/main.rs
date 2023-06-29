use anyhow::{format_err, Result};
use hyprland::{data::Client, shared::HyprDataActiveOptional};
use psutil::process::Process;
use std::{
    env::{args, set_current_dir},
    path::PathBuf,
    process::Command,
};

fn main() -> Result<()> {
    let mut args: Box<[String]> = args().skip(1).collect();
    let dir = Client::get_active().ok().flatten().and_then(|client| {
        if client.title.contains("Zellij") {
            println!("{}", args.len());
            if args.len() == 1 {
                args = ["zellij", "action", "new-pane"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
            } else {
                args = ["zellij", "run", "--"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .chain(args.into_iter().map(|s| s.clone()).skip(1))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
            }
            None
        } else {
            get_dir(client)
        }
    });
    spawn(dir, &args)?;
    Ok(())
}

fn get_dir(client: Client) -> Option<PathBuf> {
    let (process_children, process_parents) = searchable_processes()?;
    assert_eq!(process_children.len(), process_parents.len());
    get_child_cwd(client.pid as u32, &process_parents, &process_children, 0).0
}

fn searchable_processes() -> Option<(Vec<u32>, Vec<u32>)> {
    let processes = psutil::process::processes().ok()?;
    let mut processes_with_parent: Vec<_> = processes
        .iter() // Just give us the ones with valid data
        .flat_map(|process_result| process_result.as_ref().ok())
        .flat_map(|process| {
            process
                .ppid()
                .ok()
                .map(|opt_ppid| (opt_ppid, process.pid()))
        })
        .flat_map(|(opt_ppid, pid)| opt_ppid.map(|ppid| (pid, ppid)))
        .collect();
    processes_with_parent.sort_by_key(|(_pid, ppid)| *ppid);
    let process_children: Vec<_> = processes_with_parent
        .iter()
        .map(|(pid, _ppid)| *pid)
        .collect();
    let process_parents: Vec<_> = processes_with_parent
        .iter()
        .map(|(_pid, ppid)| *ppid)
        .collect();
    Some((process_children, process_parents))
}

fn get_child_cwd(
    process: u32,
    all_parents: &[u32],
    all_children: &[u32],
    depth: i32,
) -> (Option<PathBuf>, i32) {
    let children = get_children(process, all_parents, all_children);
    match &children[..] {
        &[] => (process_wd(process), 0),
        &[only] => get_child_cwd(only, &all_parents, &all_children, depth + 1),
        _ => children
            .iter()
            .map(|p| get_child_cwd(*p, &all_parents, &all_children, depth + 1))
            .max_by_key(|(_p, d)| *d)
            .unwrap(), // Empty case already handled
    }
}

fn process_wd(process: u32) -> Option<PathBuf> {
    Process::new(process).ok()?.cwd().ok()
}

fn get_children(process: u32, all_parents: &[u32], all_children: &[u32]) -> Vec<u32> {
    let lower_bound = all_parents.partition_point(|ele| ele < &process);
    let upper_bound = all_parents.partition_point(|ele| ele <= &process);
    all_children[lower_bound..upper_bound].to_owned()
}

fn spawn(path: Option<PathBuf>, args: &[String]) -> Result<()> {
    path.map(|path| set_current_dir(&path));
    args.iter()
        .skip(1)
        .fold(
            Command::new(&args.first().ok_or(format_err!("Pass a program to run"))?),
            |mut command, arg| {
                command.arg(arg);
                command
            },
        )
        .spawn()?
        .wait()?;
    Ok(())
}
