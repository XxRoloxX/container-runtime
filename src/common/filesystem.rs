use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    os::{
        linux::fs::MetadataExt,
        unix::fs::{symlink, FileTypeExt, PermissionsExt},
    },
    path::PathBuf,
};

use log::{error, info};
use nix::sys::stat::mknod;
use nix::sys::stat::Mode;
use nix::{
    mount::{mount, MsFlags},
    sys::stat::SFlag,
};

fn change_current_dir(path: &str) -> Result<(), String> {
    std::env::set_current_dir(path).map_err(|e| format!("{}", e))?;
    Ok(())
}
pub fn copy_directory(src: &str, dest: &str) -> Result<(), String> {
    fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create directory at {}: {}", dest, e))?;

    for entry in
        fs::read_dir(src).map_err(|e| format!("Failed to read directory at {}: {}", src, e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        let file_name = path.file_name().ok_or("Failed to get file name")?;
        let dest_path = PathBuf::from(dest).join(file_name);

        if path.is_symlink() {
            //Get only path of the target of the symlink without following the link
            let link = fs::read_link(&path)
                .map_err(|e| format!("Failed to read symlink at {}: {}", path.display(), e))?;

            symlink(link, &dest_path).map_err(|e| {
                format!("Failed to create symlink at {}: {}", dest_path.display(), e)
            })?;
        } else if path.is_dir() {
            copy_directory(
                path.to_str().ok_or("Failed to get file name")?,
                &dest_path.to_str().ok_or("Failed to get file name")?,
            )?;
        } else {
            let meta = fs::metadata(&path)
                .map_err(|e| format!("Failed to get metadata for {}: {}", path.display(), e))?;

            let file_type = meta.file_type();

            if file_type.is_char_device() {
                // Get Minor and Major numbers of file devices with stat
                let st_dev = meta.st_rdev();

                let major = nix::sys::stat::major(st_dev);
                let minor = nix::sys::stat::minor(st_dev);
                let permissions = Mode::from_bits_truncate(meta.st_mode());
                let flags = SFlag::S_IFCHR;

                let dev = nix::sys::stat::makedev(major, minor);

                mknod(&dest_path, flags, permissions, dev).map_err(|e| {
                    format!(
                        "Failed to create device file at {}: {}",
                        dest_path.display(),
                        e
                    )
                })?;
            } else {
                fs::copy(path.clone(), &dest_path).map_err(|e| {
                    format!("Failed to copy file from {} to : {}", path.display(), e)
                })?;
            }
        }

        info!("Copied {} to {}", path.display(), dest_path.display());
    }

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
        Err(e) => error!("Failed to remove directory at {}: {}", path, e),
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
