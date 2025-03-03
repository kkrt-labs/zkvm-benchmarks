#!/bin/bash
set -e

# --- System update and installation of essential packages ---
# "build-essential" includes make, gcc, g++ (C++ compiler), and other development tools.
apt-get update
apt-get upgrade -y
apt-get install -y build-essential curl wget git

# --- Clone the repository ---
cd /root
git clone https://github.com/grandchildrice/zkvm-benchmarks.git

# --- Setup CUDA 12.5 ---
# Download the local installer for NVIDIA CUDA 12.5 and install the toolkit non-interactively.
wget https://developer.download.nvidia.com/compute/cuda/12.5.0/local_installers/cuda_12.5.0_515.65.01_linux.run
chmod +x cuda_12.5.0_515.65.01_linux.run
./cuda_12.5.0_515.65.01_linux.run --silent --toolkit

# Set CUDA environment variables (applied system-wide)
cat << 'EOF' > /etc/profile.d/cuda.sh
export PATH=/usr/local/cuda-12.5/bin${PATH:+:${PATH}}
export LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}
EOF
chmod +x /etc/profile.d/cuda.sh
source /etc/profile.d/cuda.sh

# --- Install Rust & Cargo (nightly) ---
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env

# --- Install each project ---

## 1. Install Jolt/Nexus (add riscv32i target)
rustup target add riscv32i-unknown-none-elf

## 2. Install Risc Zero
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install

## 3. Install SP1
curl -L https://sp1.succinct.xyz | bash
# Add the sp1 binary to PATH (consider adding to ~/.bashrc for future sessions)
export PATH="$PATH:$HOME/.sp1/bin"
sp1up

## 4. Install zkm
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zkMIPS/toolchain/refs/heads/main/setup.sh | sh

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
echo 'export LD_LIBRARY_PATH=/usr/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc

source ~/.bashrc

# --- Completion message ---
echo "Setup complete. Please reboot if necessary."