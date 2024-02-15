!#/bin/bash

echo "Installing dependencies"
pacman -Sy
pacman -S --noconfirm strace debootstrap

echo "Set executable scripts"
chmod +x install_scripts/install.sh

echo "Installing the container-runtime"
./install_scripts/install.sh
