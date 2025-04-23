This repo is used for academic workshop, with the title "Benchmarking zkVMs: Efficiency, Bottlenecks, and Best Practices", at [ZKProof7 in Sofia](https://zkproof.org/events/zkproof-7-sofia/).

# zkVM Benchmarks

See bench result [here](./benchmark_outputs)

## Current status

We are so welcome to your contribution!

||Fibonacci|SHA2|ECDSA|ETHTransfer|Notes|
|-|-|-|-|-|-|
|[Jolt](./jolt/)|✅|✅|✅|✅||
|[Jolt (GPU)](./jolt/)|✅|✅|✅|✅|Optimization by GPU is still only partially implemented|
|[Nexus](./nexus/)|✅|✅|✅|❌||
|[Novanet](./novanet/)|✅|❌|❌|❌||
|[OpenVM](./openvm/)|✅|✅|✅|✅||
|[Pico](./pico/)|✅|✅|✅|✅||
|[Powdr](./powdr/)|❌|❌|❌|❌||
|[RISC Zero](./risczero)|✅|✅|✅|✅||
|[RISC Zero (GPU)](./risczero)|✅|✅|✅|✅||
|[SP1](./sp1-turbo)|✅|✅|✅|✅||
|[SP1 (GPU)](./sp1-turbo)|✅|✅|✅|✅||
|[ZKM](./zkm)|✅|✅|✅|✅||
|Ceno|||||[No SDK yet](https://github.com/orgs/scroll-tech/projects/20)|
|zkWASM||||||
|Valida|||||[The repo is currently private, and the latest docker image is not available.](https://github.com/lita-xyz/valida-releases)|
|Snarkify||||||

## Hardware Requirement

Architecture: Linux x86_64
(When I tried to build on my Mac/M1 using docker, some toolchains were unable to build.)

||CPU Cores|RAM|GPU RAM|
|-|-|-|-|
|Jolt|8|100GB|8GB|
|Nexus|8|32GB|-|
|Novanet|8|32GB|-|
|OpenVM|8|32GB|-|
|Pico|8|32GB|-|
|RISC Zero|8|32GB|8GB|
|SP1|8|64GB|24GB|
|ZKM|8|80GB|-|

## Installation

### Install Jolt/Nexus

```bash
rustup target add riscv32i-unknown-none-elf
```

### Install Nexus

```bash
rustup run nightly-2025-01-02 cargo install --git https://github.com/nexus-xyz/nexus-zkvm cargo-nexus --tag 'v0.3.1'
```

### Install Risc Zero

```bash
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

### Install SP1

```bash
curl -L https://sp1.succinct.xyz | bash
```

Follow the instructions outputted by this command then run:

```bash
sp1up
```

### Install zkm

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zkMIPS/toolchain/refs/heads/main/setup.sh | sh
source ~/.zkm-toolchain/env
```

When Open SSL error occurs:

```bash
wget https://github.com/openssl/openssl/releases/download/openssl-3.3.2/openssl-3.3.2.tar.gz -O openssl-3.3.2.tar.gz
tar -xvzf openssl-3.3.2.tar.gz
cd openssl-3.3.2
./config --prefix=/usr zlib-dynamic --openssldir=/etc/ssl shared
make test
sudo make install
export LD_LIBRARY_PATH=/usr/lib64:$LD_LIBRARY_PATH
```

### Install zkm2

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zkMIPS/toolchain/refs/heads/main/setup.sh | sh
```

- Modify ~/.cargo/config:

```bash
[target.mips-unknown-linux-musl]
linker = "<path-to>/mips-linux-muslsf-cross/bin/mips-linux-muslsf-gcc"
rustflags = ["--cfg", 'target_os="zkvm"',"-C", "target-feature=+crt-static", "-C", "link-arg=-g"]
```

### Install Open VM

```bash
cargo +nightly install --git http://github.com/openvm-org/openvm.git cargo-openvm
cargo openvm --version
```

### Install Novanet

Novanet (zkEngine) requires the wasm2wat tool to build a guest programs into WebAssembly Text Fromat.
https://github.com/WebAssembly/wabt?tab=readme-ov-file#installing-prebuilt-binaries

and install wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

#### MacOS
```bash
brew install wabt
```

#### Linux
```bash
sudo apt install wabt
```

#### Others
Please build `wabt` from [here](https://github.com/WebAssembly/wabt) on your own.

### Install Pico
```bash
rustup install nightly-2024-11-27
rustup component add rust-src --toolchain nightly-2024-11-27
cargo +nightly-2024-11-27 install --git https://github.com/brevis-network/pico pico-cli
```

## Running

To run all benchmarks run:

```bash
make bench-all
```

The benchmark results should be outputted in CSV form in `benchmark_outputs`.

To run an individual benchmark run `make bench-zkm`, `make bench-jolt`, `make bench-risczero` or ... .

## Profiling

First, you need to install these tools:
- flamegraph

Then, run this command:

```
make perf-all
```

## Disclaimer

_This code is being provided as is. No guarantee, representation or warranty is being made, express or implied, as to the safety or correctness of the code. It has not been audited and as such there can be no assurance it will work as intended, and users may experience delays, failures, errors, omissions or loss of transmitted information. Nothing in this repo should be construed as investment advice or legal advice for any particular facts or circumstances and is not meant to replace competent counsel. It is strongly advised for you to contact a reputable attorney in your jurisdiction for any questions or concerns with respect thereto. a16z is not liable for any use of the foregoing, and users should proceed with caution and use at their own risk. See a16z.com/disclosures for more info._

## Submission Guide

If you want to add new zkVM schemes for this benchmark, please follow them:

1. Submit issue
2. Assign
3. Create PR
