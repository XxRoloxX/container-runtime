use std::path::Path;

use nix::{
    mount::umount,
    sched::{unshare, CloneFlags},
    unistd::{fork, ForkResult},
};

use crate::{
    image::Image,
    rootfs::{clear_directory, mount_overlayfs, setup_rootfs},
    unshare::{execute_command, get_install_path, wait_for_child_process},
};

pub struct Container {
    id: String,
    image: Box<Image>,
    pub command: String,
    pub args: Vec<String>,
}

impl Container {
    pub fn new(id: String, image: Image, command: String, args: Vec<String>) -> Container {
        Container {
            id,
            image: Box::from(image),
            command,
            args,
        }
    }

    pub unsafe fn start(&self) -> Result<(), String> {
        self.mount_overlayfs()?;
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
                self.clean_up_on_exit()?;
                Ok(())
            }
            Ok(ForkResult::Child) => {
                self.setup_rootfs()?;
                execute_command(&self.command, self.args.iter().map(AsRef::as_ref).collect())?;
                Ok(())
            }
            Err(_) => Err("Failed to fork".to_string()),
        }
    }
    fn clean_up_on_exit(&self) -> Result<(), String> {
        let proc_mount = self.get_container_proc_mount()?;
        let overlay_mount = self.get_merged_overlayfs_path()?;

        umount(proc_mount.as_str())
            .map_err(|e| format!("Failed to umount /proc on {}: {}", proc_mount, e))?;

        umount(overlay_mount.as_str())
            .map_err(|e| format!("Failed to umount overlay on {}: {}", overlay_mount, e))?;

        println!("Cleaned on exit!");
        Ok(())
    }
    fn prepare_container_directories(&self) -> Result<(), String> {
        let work_layer_path = self.get_work_overlayfs_path()?;
        let upper_layer_path = self.get_upper_overlayfs_path()?;
        let merged_layer_path = self.get_merged_overlayfs_path()?;
        clear_directory(&work_layer_path)?;
        clear_directory(&upper_layer_path)?;
        clear_directory(&merged_layer_path)?;
        Ok(())
    }
}

impl Container {
    fn get_inner_overlay_path(&self, dirname: &str) -> Result<String, String> {
        let overlay_path = self.get_overlayfs_path()?;
        let overlay_path_builder = Path::new(&overlay_path).join(dirname);
        let overlay_path = overlay_path_builder.to_str().ok_or_else(|| {
            format!(
                "Failed to access overlay path of {} on {}",
                self.id, overlay_path
            )
        })?;
        Ok(overlay_path.to_string())
    }
    fn get_overlayfs_path(&self) -> Result<String, String> {
        let container_path = self.get_conatiner_path()?;
        let overlay_path = Path::new(&container_path).join("overlay");
        match overlay_path.to_str() {
            None => Err(format!(
                "Failed to access overlay path of {} on {}",
                self.id, container_path
            )),
            Some(path) => Ok(path.to_string()),
        }
    }
    fn get_lower_overlayfs_path(&self) -> Result<String, String> {
        self.image.get_image_path()
    }

    fn get_work_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("work")
    }
    fn get_merged_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("merged")
    }
    fn get_upper_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("upper")
    }
    fn mount_overlayfs(&self) -> Result<(), String> {
        self.prepare_container_directories()?;
        let lower = self.get_lower_overlayfs_path()?;
        let upper = self.get_upper_overlayfs_path()?;
        let work = self.get_work_overlayfs_path()?;
        let target = self.get_merged_overlayfs_path()?;
        mount_overlayfs(&lower, &upper, &work, &target);
        Ok(())
    }
    fn setup_rootfs(&self) -> Result<(), String> {
        let new_root = self.get_merged_overlayfs_path()?;
        setup_rootfs(&new_root)?;
        Ok(())
    }

    fn get_containers_path() -> Result<String, String> {
        let install_path = get_install_path()?;
        Ok(Path::new(&install_path)
            .join("containers")
            .to_str()
            .ok_or_else(|| "Failed to access containers path".to_string())?
            .to_string())
    }
    fn get_conatiner_path(&self) -> Result<String, String> {
        let containers_path = Self::get_containers_path()?;
        let container_path = Path::new(&containers_path).join(&self.id);
        match container_path.to_str() {
            None => Err(format!("Failed to access container path of {}", self.id)),
            Some(path) => Ok(path.to_string()),
        }
    }

    fn get_container_proc_mount(&self) -> Result<String, String> {
        let merged_overlay_path = self.get_merged_overlayfs_path()?;
        let proc_mount = Path::new(&merged_overlay_path).join("proc");
        match proc_mount.to_str() {
            Some(path) => Ok(path.to_string()),
            None => Err(format!("Failed to find /proc mount in {}", self.id)),
        }
    }
}
