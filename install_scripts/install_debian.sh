!#/bin/bash

echo "Installing dependencies"
apt-get update
apt-get install -y strace debootstrap

echo "Set executable scripts"
chmod +x install_scripts/install.sh

echo "Installing the container-runtime"
./install.sh

