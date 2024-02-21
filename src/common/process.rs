use nix::sched::{unshare, CloneFlags};
use nix::sys::signal::{signal, SigHandler, Signal};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, Pid};
use std::ffi::CString;

use super::commands::runtime_commands::NetworkConfiguration;

pub fn get_install_path() -> Result<String, String> {
    let install_path = env!("INSTALL_PATH").to_string();
    match install_path.as_str() {
        "" => Err("INSTALL_PATH is not set".to_string()),
        path => Ok(path.to_string()),
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

pub fn container_unshare(network: &NetworkConfiguration) -> Result<(), String> {
    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | map_network_configuration_to_unshare_flag(network),
    )
    .map_err(|e| format!("Failed to unshare: {}", e))?;
    Ok(())
}
pub fn map_network_configuration_to_unshare_flag(network: &NetworkConfiguration) -> CloneFlags {
    match network {
        NetworkConfiguration::None => CloneFlags::CLONE_NEWNET,
        NetworkConfiguration::Host => CloneFlags::empty(),
    }
}

pub fn ignore_process_termination() -> Result<(), String> {
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigIgn)
            .map_err(|e| format!("Failed to ignore SIGTERM: {}", e))?;
    }
    Ok(())
}

pub fn clear_process_signal_handlers() -> Result<(), String> {
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigDfl)
            .map_err(|e| format!("Failed to clear SIGTERM handler: {}", e))?;
    }
    Ok(())
}

pub fn kill_process(pid: Pid) -> Result<(), String> {
    nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGKILL)
        .map_err(|e| format!("Failed to kill process {}: {}", pid, e))?;
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
