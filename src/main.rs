extern crate clap;
extern crate libc;
extern crate nix;
// use clap::{App, Arg, SubCommand};
use clap::{App, Arg, SubCommand};
use container_runtime::unshare::run_container;
use std::{fs::read_dir, path::Path};

fn deploy_container(path: &str) {
    let new_root = Path::new("newroot/bin");
    let deploy_path = new_root.join(Path::new(path).file_name().unwrap());

    std::fs::copy(path, &deploy_path).expect("Failed to copy file");

    println!("Deployed to {:?}", deploy_path);
}
// fn is_proc_mounted() -> bool {
//     let file = match File::open("/proc/mounts") {
//         Ok(f) => f,
//         Err(_) => return false,
//     };
//
//     let reader = BufReader::new(file);
//
//     for line in reader.lines() {
//         if let Ok(l) = line {
//             let parts: Vec<&str> = l.split_whitespace().collect();
//             if parts.len() > 1 && parts[1] == "/proc" {
//                 return true;
//             }
//         }
//     }
//     return false;
// }

// unsafe fn run_container(cmd: &str, args: Vec<&str>) {
//     unshare(
//         CloneFlags::CLONE_NEWPID
//             | CloneFlags::CLONE_NEWNS
//             | CloneFlags::CLONE_NEWUTS
//             | CloneFlags::CLONE_NEWNET,
//     )
//     .expect("Failed to unshare");
//
//     match fork() {
//         Ok(ForkResult::Parent { child, .. }) => {
//             waitpid(child, None).expect("Failed to wait for child");
//         }
//         Ok(ForkResult::Child) => {
//             let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
//             let c_args: Vec<CString> = args
//                 .iter()
//                 .map(|arg| CString::new(*arg).expect("Failed to convert to CString"))
//                 .collect();
//             let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();
//             let current_dir = std::env::current_dir().unwrap();
//             setup_rootfs(&format!("{}/newroot", current_dir.display()));
//             println!("Running command: {}", cmd);
//             println!("Args: {:?}", &c_args_refs);
//             list_files();
//             execvp(&c_cmd, &c_args_refs).expect("Failed to execvp");
//         }
//         Err(err) => {
//             eprintln!("Fork failed! {}", err)
//         }
//     }
// }
fn main() {
    let matches = App::new("container-runtime")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a command in an isolated container")
                .arg(Arg::with_name("COMMAND").required(true).index(1))
                .arg(Arg::with_name("ARGS").multiple(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploys a file or directory to the container root")
                .arg(Arg::with_name("PATH").required(true).index(1)),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("run") {
        let cmd = matches.value_of("COMMAND").unwrap();
        let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
        unsafe {
            run_container(cmd, args);
        }
    } else if let Some(matches) = matches.subcommand_matches("deploy") {
        let path = matches.value_of("PATH").unwrap();
        deploy_container(path);
    }
}
