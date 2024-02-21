!#/bin/bash

echo "Installing dependencies"
apt-get update
apt-get install -y strace debootstrap

echo "Set executable scripts"
chmod +x install_scripts/install.sh
chmod +x install_scripts/build.sh

