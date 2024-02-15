fn main() {
    dotenv::dotenv().ok();
    let install_path =
        std::env::var("INSTALL_PATH").unwrap_or("/var/lib/container-runtime".to_string());
    println!("cargo:rustc-env=INSTALL_PATH={}", install_path);
}
