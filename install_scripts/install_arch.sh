!#/bin/bash

echo "Installing dependencies"
pacman -Sy
pacman -S --noconfirm strace debootstrap

echo "Set executable scripts"
chmod +x install_scripts/install.sh
chmod +x install_scripts/build.sh

