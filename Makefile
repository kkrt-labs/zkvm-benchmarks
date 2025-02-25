bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-powdr
	make bench-openvm

bench-jolt:
	cd jolt && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa

bench-jolt-gpu:
	cd jolt && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci -F icicle && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 -F icicle && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa -F icicle

bench-sp1:
	make build-sp1
	cd sp1 && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1-turbo:
	cd sp1-turbo && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p ecdsa-script

bench-sp1-turbo-gpu:
	cd sp1-turbo && \
	SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script

bench-zkm:
	# rust toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	cd zkm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin fibo --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin sha2 --release

bench-zkm-ecdsa:
	# rust toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	cd zkm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin ecdsa --release

bench-zkm2:
	cd zkm2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin fibonacci --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin sha2 --release

build-sp1:
	cd sp1/fibonacci && cargo prove build
	cd sp1/sha2-chain && cargo prove build
	cd sp1/sha3-chain && cargo prove build
	cd sp1/sha2 && cargo prove build
	cd sp1/sha3 && cargo prove build
	cd sp1/bigmem && cargo prove build

bench-risczero:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/fib_risczero.csv fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/sha2_risczero.csv big-sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/ecdsa_risczero.csv ecdsa-verify

bench-risczero-gpu:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/fib_risczero_gpu.csv fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/sha2_risczero_gpu.csv big-sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/ecdsa_risczero_gpu.csv ecdsa-verify

bench-powdr:
	cd powdr && RUSTFLAGS='-C target-cpu=native' cargo run --release

bench-openvm:
	cd openvm && \
	cargo openvm build && \
	cargo openvm keygen && \
	OPENVM_FAST_TEST=1 cargo openvm prove app --input "0x0A00000000000000"

bench-nexus:
	cd nexus && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-novanet:
	cd novanet && \
	RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 50 90 --wat fib/fib.wat && \
	RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 50 90 --compress --wat fib/fib.wat

perf-all:
	make perf-sp1turbo

perf-sp1turbo:
	cd sp1-turbo && \
	CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-C target-cpu=native" \
	cargo flamegraph --release -p fibonacci-script -F 100 -o ../benchmark_outputs/flamegraph_sp1turbo.svg --no-inline -- --once

perf-risczero:
	cd risczero && \
	CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-C target-cpu=native" \
	cargo flamegraph --release -F 100 -o ../benchmark_outputs/flamegraph_risczero.svg --no-inline -- profiling

perf-jolt:
	cd jolt && \
	CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-C target-cpu=native" \
	cargo flamegraph --release -p jolt-benchmarks -F 100 -o ../benchmark_outputs/flamegraph_jolt.svg --no-inline --bin profiling

perf-nexus:
	cd nexus && \
	CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-C target-cpu=native" \
	RUSTFLAGS="-C target-cpu=native" cargo flamegraph --release -p nexus -F 100 -o ../benchmark_outputs/flamegraph_nexus.svg --no-inline -- --once

perf-zkm:
	cd zkm && \
	CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-C target-cpu=native" \
	RUSTFLAGS="-C target-cpu=native" cargo flamegraph --release -p zkm-script -F 100 -o ../benchmark_outputs/flamegraph_zkm.svg --no-inline -- --once
