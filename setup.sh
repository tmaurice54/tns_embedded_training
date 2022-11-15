#!/bin/bash

echo "downloading the toolchain..."
wget -O gcc-arm-none-eabi.tar.xz https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-bet1/binrel/arm-gnu-toolchain-12.2.mpacbti-bet1-x86_64-arm-none-eabi.tar.xz

echo "Installing the toolchain..."
mkdir -p /opt/gcc-arm-none-eabi
tar xf ./gcc-arm-none-eabi.tar.xz --strip-components=1 -C /opt/gcc-arm-none-eabi
echo "export PATH=$PATH:/opt/gcc-arm-none-eabi/bin" | sudo tee -a /etc/profile.d/gcc-arm-none-eabi.sh

echo "Removing the archive file..."
rm -rf gcc-arm-none-eabi.tar.xz