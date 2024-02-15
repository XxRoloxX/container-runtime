use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, Pid};
use std::ffi::CString;

pub fn get_install_path() -> Result<String, String> {
    match std::env::var("INSTALL_PATH") {
        Ok(path) => Ok(path),
        Err(_) => Err("INSTALL_PATH not set".to_string()),
    }
}

pub fn wait_for_child_process(child: Pid) {
    waitpid(child, None).expect("Failed to wait for child");
}

pub fn execute_command(cmd: &str, args: Vec<String>) -> Result<(), String> {
    let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
    let c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).expect("Failed to convert to CString"))
        .collect();
    let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();
    execvp(&c_cmd, &c_args_refs).map_err(|e| format!("Failed to execute command:{} {}", cmd, e))?;
    Ok(())
}

pub fn container_unshare() -> Result<(), String> {
    unshare(
        CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS, // | CloneFlags::CLONE_NEWNET,
    )
    .map_err(|e| format!("Failed to unshare: {}", e))?;
    Ok(())
}

pub fn redirect_standard_output(output_file_descriptor: i32) -> Result<(), String> {
    nix::unistd::dup2(output_file_descriptor, nix::libc::STDOUT_FILENO).map_err(|e| {
        format!(
            "Failed to redirect stdout {}: {}",
            output_file_descriptor, e
        )
    })?;
    Ok(())
}
