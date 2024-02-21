echo "Loading environment variables"
# Load .env file form the parent directory
source ./.env

echo $INSTALL_PATH

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
touch $BASE_IMAGE_PATH/entrypoints.json

echo "Copy logger configuration"
cp ./log4rs.yaml $INSTALL_PATH

echo "Installing the project dependencies"
cp -rf ./target/release /opt/container-runtime

echo "Creating the symlinks to the binaries"
ln -snf /opt/container-runtime/daemon /usr/local/bin/container-runtimed
ln -snf /opt/container-runtime/client /usr/local/bin/container-runtime

echo "Installing the daemon systemd service"
cp ./install_scripts/container-runtime.service /etc/systemd/system/container-runtimed.service

