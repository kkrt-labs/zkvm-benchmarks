This repo is used for academic workshop, with the title "Benchmarking zkVMs: Efficiency, Bottlenecks, and Best Practices", at [ZKProof7 in Sofia](https://zkproof.org/events/zkproof-7-sofia/).

# zkVM Benchmarks

See bench result [here](./outputs/benchmark)

## Prerequisite

- [Rust](https://www.rust-lang.org/tools/install)
- [Cuda Toolkit](https://developer.nvidia.com/cuda-toolkit)
- [Cuda Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)

## Running

### Option 1. Local

Firstly, install all toolchains:

- Jolt: https://jolt.a16zcrypto.com/usage/quickstart.html#installing
- Nexus: https://docs.nexus.xyz/zkvm/proving/sdk#1-install-the-nexus-zkvm
- RISC Zero: https://dev.risczero.com/api/zkvm/install
- SP1: https://docs.succinct.xyz/docs/sp1/getting-started/install
- ZKM: https://docs.zkm.io/introduction/installation.html
- OpenVM: https://book.openvm.dev/getting-started/install.html
- Pico: https://docs.brevis.network/getting-started/installation

Then, run:

```bash
make bench-all
```

or

```bash
make bench-<sp1|sp1-gpu|jolt|jolt-gpu|nexus|openvm|pico|risczero|risczero-gpu|zkm>
```

### Option 2. Docker

```
cd docker
docker compose build base
docker compose run <sp1|jolt|nexus|openvm|pico|risczero|zkm>
```

## Current status

We are so welcome to your contribution!

||Fibonacci|SHA2|ECDSA|ETHTransfer|Notes|
|-|-|-|-|-|-|
|[SP1 (CPU/GPU)](./sp1)|✅|✅|✅|✅||
|[RISC Zero (CPU/GPU)](./risczero)|✅|✅|✅|✅||
|[ZKM](./zkm)|✅|✅|✅|✅||
|[Novanet](./novanet/)|✅|❌|❌|❌||
|Ceno|||||[No SDK yet](https://github.com/orgs/scroll-tech/projects/20)|
|[OpenVM](./openvm/)|✅|✅|✅|✅||
|[Jolt (CPU/GPU)](./jolt/)|✅|✅|✅|✅|It's still in development. GPU Acceleration is still only partially implemented.|
|[Nexus](./nexus/)|✅|✅|❌|❌|`std` is not supported yet.|
|[Pico](./pico/)|✅|✅|✅|✅||
|zkWASM||||||
|Valida|||||[The repo is currently private, and the latest docker image is not available.](https://github.com/lita-xyz/valida-releases)|
|Snarkify||||||
|Zisk||||||

## Hardware Requirement

|||
|-|-|
|Architecture|Linux x86_64|
|vCPU|8|
|RAM|128 GB|
|VRAM|24GB|
