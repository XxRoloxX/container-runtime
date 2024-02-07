// use clap::{App, Arg, SubCommand};
//
// extern crate clap;
use crate::rootfs::setup_rootfs;
use nix::errno::Errno;
use nix::mount::{mount, MsFlags};
use nix::sched::{clone, unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult, Pid};
use std::ffi::CString;
use std::fs::read_dir;

pub unsafe fn run_container(cmd: &str, args: Vec<&str>) {
    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWNET,
    )
    .expect("Failed to unshare");

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            wait_for_child_process(child);
        }
        Ok(ForkResult::Child) => {
            let current_dir = std::env::current_dir().unwrap();
            setup_rootfs(&format!("{}/newroot", current_dir.display()));
            list_files();
            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    wait_for_child_process(child);
                }
                Ok(ForkResult::Child) => {
                    execute_command(cmd, args);
                }
                Err(err) => {
                    fork_failed(err);
                }
            }
        }
        Err(err) => {
            fork_failed(err);
        }
    }
}

fn wait_for_child_process(child: Pid) {
    waitpid(child, None).expect("Failed to wait for child");
}
fn fork_failed(err: Errno) {
    eprintln!("Fork failed! {}", err);
}

fn list_files() {
    let paths = read_dir("../").unwrap();
    for path in paths {
        let p = path.unwrap().path();
        println!("Name: {}", p.display());
    }
}
pub fn execute_command(cmd: &str, args: Vec<&str>) {
    let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
    let c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(*arg).expect("Failed to convert to CString"))
        .collect();
    let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();
    println!("Running command: {}", cmd);
    println!("Args: {:?}", &c_args_refs);
    execvp(&c_cmd, &c_args_refs).expect("Failed to execvp");
}
