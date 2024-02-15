use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    os::{
        fd::AsRawFd,
        linux::fs::MetadataExt,
        unix::fs::{symlink, FileTypeExt},
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

pub enum FileType {
    Directory,
    Symlink,
    Device,
    File,
}

impl FileType {
    pub fn from_path(path: &str) -> Result<FileType, String> {
        let path = PathBuf::from(path);

        if path.is_symlink() {
            return Ok(FileType::Symlink);
        };

        let meta = path.metadata().map_err(|e| {
            format!(
                "Failed to get metadata for {}: {}",
                path.to_str().unwrap(),
                e
            )
        })?;

        if path.is_dir() {
            Ok(FileType::Directory)
        } else if meta.file_type().is_char_device() {
            Ok(FileType::Device)
        } else {
            Ok(FileType::File)
        }
    }
}

pub fn change_current_dir(path: &str) -> Result<(), String> {
    std::env::set_current_dir(path).map_err(|e| format!("{}", e))?;
    Ok(())
}

pub fn copy_directory(src: &str, dest: &str) -> Result<(), String> {
    match FileType::from_path(src) {
        Ok(FileType::Symlink) => {
            copy_symlink(src, dest)?;
        }
        Ok(FileType::Device) => {
            copy_device_file(SFlag::S_IFCHR, src, dest)?;
        }
        Ok(FileType::File) => {
            copy_standard_file(src, dest)?;
        }
        Ok(FileType::Directory) => {
            fs::create_dir_all(dest)
                .map_err(|e| format!("Failed to create directory at {}: {}", dest, e))?;

            for entry in fs::read_dir(src)
                .map_err(|e| format!("Failed to read directory at {}: {}", src, e))?
            {
                let entry = (entry.map_err(|e| format!("Failed to read entry: {}", e))?).path();
                let file_name = entry.file_name().ok_or("Failed to get file name")?;
                let path = entry.to_str().ok_or("Failed to get file name")?;
                let dest_path_buf = PathBuf::from(dest).join(file_name);
                let dest_path = dest_path_buf.to_str().ok_or("Failed to get file name")?;

                copy_directory(path, dest_path)?;
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    info!("Copied {} to {}", src, dest);
    Ok(())
}

pub fn copy_symlink(src: &str, dest: &str) -> Result<(), String> {
    let link =
        fs::read_link(src).map_err(|e| format!("Failed to read symlink at {}: {}", src, e))?;

    symlink(link, dest).map_err(|e| format!("Failed to create symlink at {}: {}", dest, e))?;
    Ok(())
}

pub fn copy_device_file(flag: SFlag, src: &str, dest: &str) -> Result<(), String> {
    let meta =
        fs::metadata(src).map_err(|e| format!("Failed to get metadata for {}: {}", src, e))?;

    let st_dev = meta.st_rdev();

    let major = nix::sys::stat::major(st_dev);
    let minor = nix::sys::stat::minor(st_dev);
    let permissions = Mode::from_bits_truncate(meta.st_mode());

    let dev = nix::sys::stat::makedev(major, minor);

    mknod(dest, flag, permissions, dev)
        .map_err(|e| format!("Failed to create device file at {}: {}", dest, e))?;
    Ok(())
}

pub fn copy_standard_file(src: &str, dest: &str) -> Result<(), String> {
    fs::copy(src, dest).map_err(|e| format!("Failed to copy file from {} to : {}", src, e))?;
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
    let flags = MsFlags::MS_NOSUID | MsFlags::MS_NOATIME;
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

pub fn get_file_descriptor(file_path: &str) -> Result<i32, String> {
    let socket = std::os::unix::net::UnixStream::connect(file_path)
        .map_err(|e| format!("Failed to open socket file {}: {}", file_path, e))?;
    Ok(socket.as_raw_fd())
}
