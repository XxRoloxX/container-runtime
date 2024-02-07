use std::path::Path;

pub fn deploy_container(path: &str) {
    let new_root = Path::new("newroot/bin");
    let deploy_path = new_root.join(Path::new(path).file_name().unwrap());

    std::fs::copy(path, &deploy_path).expect("Failed to copy file");

    println!("Deployed to {:?}", deploy_path);
}
