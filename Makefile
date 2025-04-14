bench-all:
	make bench-jolt
	make bench-jolt-gpu
	make bench-sp1-turbo
	make bench-sp1-turbo-gpu
	make bench-risczero
	make bench-risczero-gpu
	make bench-zkm
	make bench-openvm
	make bench-pico

bench-all2:
	make bench-openvm
	make bench-pico

bench-all-fibonacci:
	cd jolt && \ RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	cd ../sp1-turbo && RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --input=10 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --input=100 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --input=1000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --input=10000 && \
	cd ../risczero && RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/fib_risczero.csv fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/fib_risczero-gpu.csv fibonacci && \
	cd ../zkm && RUSTFLAGS="-C target-cpu=native" cargo run --bin fibo --release && \
	cd ../openvm && RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	cd ../novanet && RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 100 1000 10000 --wat fib/fib.wat && \
	cd ../nexus && RUSTFLAGS="-C target-cpu=native" cargo run --release && \

bench-jolt:
	cd jolt && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth

bench-jolt-gpu:
	cd jolt && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth -F icicle

bench-jolt-gpu2:
	cd jolt && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth -F icicle


bench-sp1-turbo:
	cd sp1-turbo && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p ecdsa-script && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p transfer-eth-script

bench-sp1-turbo-gpu:
	cd sp1-turbo && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --n 10 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --n 100 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --n 1000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --n 10000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p fibonacci-script -- --n 100000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script -- --n 32 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script -- --n 256 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script -- --n 512 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script -- --n 1024 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p sha2-script -- --n 2048 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p ecdsa-script && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p transfer-eth-script -- --n 1 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p transfer-eth-script -- --n 10 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p transfer-eth-script -- --n 100

bench-zkm:
	# rust toolchain path: ~/.zkm-toolchain/rust-toolchain-x86-64-unknown-linux-gnu-20241217/bin
	. ~/.zkm-toolchain/env && \
	export LD_LIBRARY_PATH=/usr/lib64:$LD_LIBRARY_PATH && \
	cd zkm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin fibo --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin sha2 --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin ecdsa --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin transfer-eth --release

bench-zkm2:
	cd zkm2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin fibonacci --release && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin sha2 --release

bench-risczero:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/fib_risczero.csv fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/sha2_risczero.csv big-sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/ecdsa_risczero.csv ecdsa-verify && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -- --out ../benchmark_outputs/ethtransfer_risczero.csv transfer-eth

bench-risczero-gpu:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/fib_risczero-gpu.csv fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/sha2_risczero-gpu.csv big-sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/ecdsa_risczero-gpu.csv ecdsa-verify && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda -- --out ../benchmark_outputs/ethtransfer_risczero-gpu.csv transfer-eth

bench-powdr:
	cd powdr && RUSTFLAGS='-C target-cpu=native' cargo run --release

bench-openvm:
	cd openvm && \
	RUST_BACKTRACE=1 RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	RUST_BACKTRACE=1 RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 && \
	RUST_BACKTRACE=1 RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa -F std && \
	RUST_BACKTRACE=1 RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth -F std

bench-nexus:
	cd nexus && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-novanet:
	cd novanet && \
	RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 100 --compress --wat fib/fib.wat

bench-pico:
	cd pico/fibonacci/app && \
	cargo pico build && \
	cd ../prover && \
	cargo run --release

	cd pico/sha2-256/app && \
	cargo pico build && \
	cd ../prover && \
	cargo run --release

	cd pico/ecdsa/app && \
	cargo pico build && \
	cd ../prover && \
	cargo run --release

	cd pico/ethblock/app && \
	cargo pico build && \
	cd ../prover && \
	cargo run --release && \
	cd ../../../..

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
