use crate::container::Container;
use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult, Pid};
use std::ffi::CString;

pub fn get_install_path() -> Result<String, String> {
    match std::env::var("INSTALL_PATH") {
        Ok(path) => Ok(path),
        Err(_) => Err("INSTALL_PATH not set".to_string()),
    }
}
pub unsafe fn run_container(container: &Container) -> Result<(), String> {
    container.mount_overlayfs()?;
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
            container.clean_up_on_exit()?;
            Ok(())
        }
        Ok(ForkResult::Child) => {
            container.setup_rootfs()?;
            execute_command(
                &container.command,
                container.args.iter().map(AsRef::as_ref).collect(),
            )?;
            Ok(())
        }
        Err(_) => Err("Failed to fork".to_string()),
    }
}

fn wait_for_child_process(child: Pid) {
    waitpid(child, None).expect("Failed to wait for child");
}

pub fn execute_command(cmd: &str, args: Vec<&str>) -> Result<(), String> {
    let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
    let c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(*arg).expect("Failed to convert to CString"))
        .collect();
    let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();
    println!("Running command: {}", cmd);
    println!("Args: {:?}", &c_args_refs);
    execvp(&c_cmd, &c_args_refs).map_err(|e| format!("Failed to execute command:{} {}", cmd, e))?;
    Ok(())
}
