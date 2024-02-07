reset:
	cd ./newroot/ && sudo umount ./proc && sudo rm -rf ./proc/ && sudo mkdir ./proc

run_rust:
	sudo ./target/debug/container-runtime run /bin/app-in-container

run_bash:
	sudo ./target/debug/container-runtime run /bin/bash
