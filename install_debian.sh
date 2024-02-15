!#/bin/bash

echo "Installing dependencies"
apt-get update
apt-get install -y strace debootstrap

echo "Loading environment variables"
source ./env.sh

CONTAINTERS_PATH=$INSTALL_PATH/containers
IMAGES_PATH=$INSTALL_PATH/images

echo "Creating the app directory"
mkdir -p $INSTALL_PATH

echo "Creating containers directory"
mkdir -p $CONTAINTERS_PATH 

echo "Creating the images directory"
mkdir -p $IMAGES_PATH 

BASE_IMAGE_PATH=$IMAGES_PATH/base

echo "Creating the base image"
mkdir -p $BASE_IMAGE_PATH

echo "Downloading the base image"
debootstrap stable $BASE_IMAGE_PATH/rootfs http://deb.debian.org/debian/

echo "Creating base image entrypoint"
touch $BASE_IMAGE_PATH/entrypoint.sh


