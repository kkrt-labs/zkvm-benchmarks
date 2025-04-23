bench-all:
	make bench-jolt
	make bench-jolt-gpu
	make bench-sp1
	make bench-sp1-gpu
	make bench-risczero
	make bench-risczero-gpu
	make bench-zkm
	make bench-openvm
	make bench-pico

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

bench-sp1:
	cd sp1 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth

bench-sp1-gpu:
	cd sp1 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib -- --n 10 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib -- --n 100 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib -- --n 1000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib -- --n 10000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib -- --n 100000 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 -- --n 32 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 -- --n 256 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 -- --n 512 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 -- --n 1024 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 -- --n 2048 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin ecdsa && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth -- --n 1 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth -- --n 10 && \
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth -- --n 100

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
	cd nexus && RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fib && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2

bench-novanet:
	cd novanet && \
	RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 100 --compress --wat fib/fib.wat

bench-pico:
	cd pico/host && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fib && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth

build-pico:
	cd pico/fibonacci-guest && \
	cargo pico build

	cd pico/sha2-guest && \
	cargo pico build

	cd pico/ecdsa-guest && \
	cargo pico build

	cd pico/transfer-eth-guest && \
	cargo pico build
