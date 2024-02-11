reset:
	cd ./newroot/ && sudo umount ./proc && sudo rm -rf ./proc/ && sudo mkdir ./proc

run_rust:
	sudo ./target/debug/container-runtime run /bin/app-in-container

run_bash:
	sudo ./target/debug/container-runtime run /bin/bash

run_daemon:
	cargo build && sudo RUST_LOG=info RUST_BACKTRACE=1 ./target/debug/daemon 
run_client:
	cargo build && sudo RUST_LOG=info ./target/debug/client $(ARGS)

start_container:
	cargo build && sudo RUST_LOG=info ./target/debug/client start  new_container new_image4 /bin/bash 

