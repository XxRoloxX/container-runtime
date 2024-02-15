run_daemon:
	cargo build && sudo RUST_LOG=info RUST_BACKTRACE=1 ./target/debug/daemon 
run_client:
	cargo build && sudo RUST_LOG=info ./target/debug/client $(ARGS)

build_image:
	cargo build && sudo RUST_LOG=info ./target/debug/client build  $(ARGS)

start_container:
	cargo build && sudo RUST_LOG=info ./target/debug/client start  new_container new_image4 /bin/bash 

install_arch:
	sudo ./install_arch.sh
install_debian:
	sudo ./install_debian.sh
	

