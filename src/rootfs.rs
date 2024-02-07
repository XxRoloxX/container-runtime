use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nix::mount::{mount, MsFlags};

pub fn setup_rootfs(new_root: &str) {
    std::env::set_current_dir(new_root).expect("Failed to change directory");
    nix::unistd::chroot(new_root).expect("Failed to change root");
    std::env::set_current_dir("/").expect("Failed to change directory");
    mount_proc();
}

fn mount_proc() {
    if !is_proc_mounted() {
        mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::empty(),
            None::<&str>,
        )
        .unwrap();
    }
}

fn is_proc_mounted() -> bool {
    let file = match File::open("/proc/mounts") {
        Ok(f) => f,
        Err(_) => return false,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() > 1 && parts[1] == "/proc" {
                return true;
            }
        }
    }
    return false;
}
