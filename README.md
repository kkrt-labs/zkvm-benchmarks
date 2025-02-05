# zkVM Benchmarks

Supported zkVMs:

- Jolt
- Nexus (WIP)
- Powdr (WIP)
- RISC Zero
- SP1

Want to support in the future:

- Ceno: insufficient impelentation, such as non-supporting recursion, [guest programs](https://github.com/orgs/scroll-tech/projects/20), etc.
- zkWASM (in the future)
- Open VM (in the future)
- Valida: [The repo is currently private, and the latest docker image is not available.](https://github.com/lita-xyz/valida-releases)
- Snarkify?

Don't support:

- Cairo VM: doesn't support Rust yet
- Miden VM: doesn't support Rust yet
- ...

## Installation

We highly recommend to build them on:
- linux/x86_64
- >64GB RAM
- strong CPU

When I tried to build on my Mac/M1 using docker, some toolchains were unable to build.

### Install Jolt/Nexus

```bash
rustup target add riscv32i-unknown-none-elf
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
wget http://musl.cc/mips-linux-muslsf-cross.tgz
tar -zxvf mips-linux-muslsf-cross.tgz
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

## Running

To run all benchmarks run:

```bash
make bench-all
```

The benchmark results should be outputted in CSV form in `benchmark_outputs`.

To run an individual benchmark run `make bench-zkm`, `make bench-jolt`, `make bench-risczero` or `make bench-sp1`.

## Disclaimer

_This code is being provided as is. No guarantee, representation or warranty is being made, express or implied, as to the safety or correctness of the code. It has not been audited and as such there can be no assurance it will work as intended, and users may experience delays, failures, errors, omissions or loss of transmitted information. Nothing in this repo should be construed as investment advice or legal advice for any particular facts or circumstances and is not meant to replace competent counsel. It is strongly advised for you to contact a reputable attorney in your jurisdiction for any questions or concerns with respect thereto. a16z is not liable for any use of the foregoing, and users should proceed with caution and use at their own risk. See a16z.com/disclosures for more info._

## Submission Guide

If you want to add new zkVM schemes for this benchmark, please follow them:

...