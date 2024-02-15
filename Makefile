run_daemon:
	cargo build && sudo RUST_LOG=info RUST_BACKTRACE=1 ./target/debug/daemon 
run_client:
	cargo build && sudo RUST_LOG=info ./target/debug/client $(ARGS)

build_image:
	cargo build && sudo RUST_LOG=info ./target/debug/client build  $(ARGS)

start_container:
	cargo build && sudo RUST_LOG=info ./target/debug/client start  new_container new_image4 /bin/bash 

install_arch:
	sudo chmod +x ./install_scripts/install_arch.sh && sudo ./install_scripts/install_arch.sh && cargo install --path . --force
install_debian:
	sudo chmod +x ./install_scripts/install_debian.sh && sudo ./install_scripts/install_debian.sh && cargo install --path . --force
install:
	cargo install --path . --force
	

