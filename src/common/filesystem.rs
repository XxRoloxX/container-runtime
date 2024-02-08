use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nix::mount::{mount, MsFlags};

fn change_current_dir(path: &str) -> Result<(), String> {
    std::env::set_current_dir(path).map_err(|e| format!("{}", e))?;
    Ok(())
}

pub fn setup_rootfs(new_root: &str) -> Result<(), String> {
    change_current_dir(new_root)?;
    nix::unistd::chroot(new_root)
        .map_err(|e| format!("Failed to chroot into {}: {}", new_root, e))?;
    change_current_dir("/")?;
    mount_proc();
    Ok(())
}
pub fn clear_directory(path: &str) -> Result<(), String> {
    match std::fs::remove_dir_all(path) {
        Ok(_) => {}
        Err(e) => println!("Failed to remove directory at {}: {}", path, e),
    };

    std::fs::create_dir_all(path)
        .map_err(|e| format!("Failed to create direcory at {}: {}", path, e))?;
    Ok(())
}

pub fn mount_overlayfs(lower: &str, upper: &str, work: &str, target: &str) {
    let flags = MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOATIME;
    mount(
        Some("overlay"),
        target,
        Some("overlay"),
        flags,
        Some(format!("lowerdir={},upperdir={},workdir={}", lower, upper, work).as_str()),
    )
    .expect("Failed to mount overlayfs");
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
