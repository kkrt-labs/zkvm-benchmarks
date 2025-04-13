#!/bin/bash
set -e

# --- System update and installation of essential packages ---
# "build-essential" includes make, gcc, g++ (C++ compiler), and other development tools.
sudo apt-get update
sudo apt-get upgrade -y
sudo apt-get install -y build-essential curl wget git libssl-dev pkg-config cmake

# --- Clone the repository ---
cd /root
git clone https://github.com/grandchildrice/zkvm-benchmarks.git

# --- Setup CUDA ^12.5 ---
sudo apt -y install ubuntu-drivers-common
sudo add-apt-repository ppa:graphics-drivers/ppa
sudo apt update
sudo apt install nvidia-driver-570
sudo reboot
sudo apt-get update
sudo apt-get install nvidia-cuda-toolkit

# --- Install Rust & Cargo (nightly) ---
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env

# --- Install each project ---

## 1. Install Jolt/Nexus (add riscv32i target)
rustup target add riscv32i-unknown-none-elf

## 2. Install Risc Zero
cargo install cargo-binstall
curl -L https://risczero.com/install | bash
source "~/.bashrc"
rzup install

## 3. Install SP1
curl -L https://sp1.succinct.xyz | bash
# Add the sp1 binary to PATH (consider adding to ~/.bashrc for future sessions)
export PATH="$PATH:$HOME/.sp1/bin"
sp1up
sudo apt-get update
sudo apt-get install docker.io
sudo systemctl start docker
sudo systemctl enable docker

curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

sudo apt-get install -y nvidia-container-toolkit
sudo systemctl restart docker
sudo usermod -aG docker $USER

## 4. Install zkm
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zkMIPS/toolchain/refs/heads/main/setup.sh | sh

## 5. Install OpenVM
cargo +nightly install --git http://github.com/openvm-org/openvm.git cargo-openvm
rustup component add rust-src --toolchain nightly-2024-10-30-x86_64-unknown-linux-gnu

# Persist zkm toolchain environment across sessions by appending to .bashrc
echo "source ~/.zkm-toolchain/env" >> ~/.bashrc

# --- Install OpenSSL 3.3.2 ---
cd /root
wget https://github.com/openssl/openssl/releases/download/openssl-3.3.2/openssl-3.3.2.tar.gz -O openssl-3.3.2.tar.gz
tar -xvzf openssl-3.3.2.tar.gz
cd openssl-3.3.2
./config --prefix=/usr zlib-dynamic --openssldir=/etc/ssl shared
make test
make install

# Persist LD_LIBRARY_PATH for OpenSSL across sessions by appending to .bashrc
sudo ln -s /usr/lib/cuda /usr/local/cuda
echo 'export CUDA_HOME=/usr/local/cuda' >> ~/.bashrc
echo 'export PATH=$CUDA_HOME/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu:$CUDA_HOME/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc

sudo apt-get install gcc-12 g++-12
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-12 100
sudo update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-12 100

# 6. Install Pico
rustup install nightly-2024-11-27
rustup component add rust-src --toolchain nightly-2024-11-27 
cargo +nightly-2024-11-27 install --git https://github.com/brevis-network/pico pico-cli

source ~/.bashrc

# --- Completion message ---
echo "Setup complete. Please reboot if necessary."