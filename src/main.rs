use clap::{App, Arg, SubCommand};
use container_runtime::container::Container;
use container_runtime::deployment::deploy_container;
use container_runtime::image::Image;
// use container_runtime::unshare::mount_overlayfs;
use container_runtime::unshare::run_container;
use dotenv::dotenv;
//
fn main() {
    dotenv().ok();
    // println!("{}", std::env::var("INSTALL_PATH").unwrap());
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
            let debug_container: Container = Container::new(
                "1".to_string(),
                Image::new("debian".to_string()),
                cmd.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            );
            run_container(&debug_container).unwrap();
        }
    } else if let Some(matches) = matches.subcommand_matches("deploy") {
        let path = matches.value_of("PATH").unwrap();
        deploy_container(path);
    }
}

// fn main() {
//     mount_overlayfs();
// }
