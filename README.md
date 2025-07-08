# ZK-VM Benchmarks

This repo is inspired by grandchildrice ZK-VMs benchmarks, based on a16z ZK-VM benchmarks.
grandchildrice benchmarks focuses on ZK-VMs capable of executing Rust programs (Jolt, Nexus, OpenVM, Pico, RiscZero, SP1, ZKM), aiming at fairness in conditions on powerful machines. The current repo extend these to ZK-VMs targeting zkDSLs, namely CairoM, Noir ProveKit and Miden.

The goal of the current benchmarks is to compare ZK-VM in a context of client-side proving on accessible devices, read laptop and mobile devices. The on chain verification is discarded and proof size is less of importance.

As the language used to define programs might not exactly have the same features, an *applicative* approach has been taken: the benchmarked program is the most optimized version for a given zkDSL/ZK-VM.
For example, the fibonacci program will use the native field of the ZK-VM when doable.

Benchmark state (prev. current status)
|ZK-VM|Fibonacci|
|-|-|
|Cairo M | ✅ |
|Miden |✅|
|Noir ProveKit|✅|
|RiscZero|✅|
|SP1|✅|

The following ZkVM are yet to be adapted: Cairo 252, jolt, nexus, Noir Barretenberg, OpenVM, Pico, ZkM, Valida, Ceno
Benchmarks can be done on ARM64 (MacOS) and x86 architectures.

## Prerequisites
- Rust

## Run Benchmarks

### Local
#### Setup ZK-VM Toolchains

Install all required toolchains:
- [R0](https://dev.risczero.com/api/zkvm/install)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)
- [Noir](https://noir-lang.org/docs/getting_started/quick_start)

#### Launch benchmark
you can either launch all benchmarks in a single command:
```bash
make bench-all
```

Or launch benchmark for a given ZK-VM:
```bash
make bench-<cairo-m|miden|noir-provekit|risczero|sp1>
```

## Benchmark Details
### Guest Programs
Not all the benchmarked ZK-VMs are working with similar guest programs.
To still compare the different projects, an applicative approach has been taken: the most optimized version of the guest program for a project is used.

#### Fibonacci
The computation of the 10, 100, 1,000, 10,000 and 100,000 terms of the Fibonacci sequence are benchmarked.

For fibonacci, the program requirements are loose so the native field of a ZK-VM can be used in zkDSL, avoiding range checks while still computing the n-th term of a fibonacci sequence.

### Security Level
To properly compare the various ZK-VM projects, all projects should have the same expected security level, expressed in bits.
The following table summarizes the expected security level of the ZK-VMs in these benchmarks.

|ZK-VM|security level (bits)|links|
|-|-|-|
|Cairo M | 96 |[link](https://github.com/kkrt-labs/cairo-m/pull/137/)|
|Miden |96|[link1](https://github.com/0xMiden/miden-vm/blob/1878ce974a7aa8834e70072b5ef3ca4d299b9873/air/src/options.rs#L182-L186), [link2](https://github.com/0xMiden/miden-vm/blob/1878ce974a7aa8834e70072b5ef3ca4d299b9873/miden-vm/src/cli/prove.rs#L60-L62)|
|Noir ProveKit|128|[link](https://github.com/worldfnd/ProveKit/blob/77304a3509554ef82025348ecbb660614ac50c0a/noir-r1cs/src/whir_r1cs.rs#L96)|
|RiscZero|96|[link](https://github.com/risc0/risc0/blob/bef7bf580eb13d5467074b5f6075a986734d3fe5/website/api/security-model.md#cryptographic-security)|
|SP1|108.5| [link1](https://docs.succinct.xyz/assets/files/SP1_Turbo_Memory_Argument-b042ba18b58c4add20a8370f4802f077.pdf), [link2](https://docs.succinct.xyz/docs/sp1/security/security-model#security-of-elliptic-curves-over-extension-fields), [link3](https://docs.succinct.xyz/docs/sp1/security/security-model#conjectures-for-fris-security)|
## Results
### Macbook M2 Max

Results can be found [here](.outputs/benchmark/simple_benchmarks.ipynb).
Note that any process on your device can influence the results.

## Contributions
If there are inconsistencies, errors, or improvements, contributions are welcome.
