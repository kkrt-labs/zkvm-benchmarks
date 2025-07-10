results_file := .outputs/simple_benchmark.ipynb

bench-all:
	make bench-cairo
	make bench-cairo-m
	make bench-miden
	make bench-noir-provekit
	make bench-openvm
	make bench-risczero
	make bench-sp1
	make bench-zkm
	@echo "Results are available through Jupyter Notebook: $(results_file)"

bench-zkm:
	. ~/.zkm-toolchain/env && \
	cd zkm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --bin fibonacci --release

bench-cairo:
	cd cairo/test_data && \
	scarb --profile release build && \
	cd ../ && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-cairo-m:
	cd cairo-m && \
	RUSTFLAGS="-C link-arg=-fuse-ld=/opt/homebrew/opt/lld/bin/ld64.lld -C target-cpu=native" cargo run --release

bench-miden:
	cd miden && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-noir-provekit:
	cd noir_provekit/runner/test_data && \
	nargo compile && \
	cd ../../ && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	cd sp1 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib

bench-risczero:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci

bench-jolt:
	cd jolt && \
	rustup override set nightly && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth

bench-jolt-gpu:
	cd jolt && \
	rustup override set nightly && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2 -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin ecdsa -F icicle && \
	ICICLE_BACKEND_INSTALL_DIR=$$(pwd)/target/release/deps/icicle/lib/backend RUSTFLAGS="-C target-cpu=native" cargo run --release --bin transfer-eth -F icicle

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
	RUST_BACKTRACE=1 SP1_PROVER=cuda RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth -- --n 100 && \
	chmod +x agg-sp1gpu-csv.sh && \
	./agg-sp1gpu-csv.sh ./.outputs/benchmark

bench-risczero-gpu:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda --bin fibonacci && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -F cuda --bin transfer-eth

bench-powdr:
	cd powdr && RUSTFLAGS='-C target-cpu=native' cargo run --release

bench-openvm:
	cd openvm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fibonacci

bench-nexus:
	cd nexus && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin fib && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2

bench-novanet:
	cd novanet && \
	RUSTFLAGS="-C target-cpu=native" RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 10 100 --compress --wat fib/fib.wat

bench-pico:
	cd pico && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin fib && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin sha2 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin ecdsa && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin transfer-eth

build-pico:
	cd pico/fibonacci-guest && \
	cargo pico build

	cd pico/sha2-guest && \
	cargo pico build

	cd pico/ecdsa-guest && \
	cargo pico build

	cd pico/transfer-eth-guest && \
	cargo pico build

perf-jolt:
	cd jolt && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./jolt/target/release/profiling \
		./.outputs/profiling/profile_jolt.pb \
		> ./.outputs/profiling/profile_jolt.svg

perf-sp1:
	cd sp1 && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./sp1/target/release/profiling \
		./.outputs/profiling/profile_sp1.pb \
		> ./.outputs/profiling/profile_sp1.svg

perf-openvm:
	cd openvm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./openvm/target/release/profiling \
		./.outputs/profiling/profile_openvm.pb \
		> ./.outputs/profiling/profile_openvm.svg


perf-nexus:
	cd nexus && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./nexus/target/release/profiling \
		./.outputs/profiling/profile_nexus.pb \
		> ./.outputs/profiling/profile_nexus.svg

perf-pico:
	cd pico && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release -p host --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./pico/target/release/profiling \
		./.outputs/profiling/profile_pico.pb \
		> ./.outputs/profiling/profile_pico.svg

perf-zkm:
	. ~/.zkm-toolchain/env && \
	cd zkm && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./zkm/target/release/profiling \
		./.outputs/profiling/profile_zkm.pb \
		> ./.outputs/profiling/profile_zkm.svg

perf-risczero:
	cd risczero && \
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin profiling

	pprof -svg \
		--ignore='rayon::.*' \
		--nodefraction=0.01 \
  		--edgefraction=0.005 \
		--compact_labels=true \
		--hide='__libc_.*' \
		./risczero/target/release/profiling \
		./.outputs/profiling/profile_risczero.pb \
		> ./.outputs/profiling/profile_risczero.svg

heap-sp1:
	cd sp1 && \
	heaptrack --output ../heaptrack.sp1.gz ./target/release/profiling

heap-openvm:
	cd openvm && \
	heaptrack --output ../heaptrack.openvm.gz ./target/release/profiling

heap-nexus:
	cd nexus && \
	heaptrack --output ../heaptrack.nexus.gz ./target/release/profiling

heap-pico:
	cd pico && \
	heaptrack --output ../heaptrack.pico.gz ./target/release/profiling

heap-zkm:
	cd zkm && \
	heaptrack --output ../heaptrack.zkm.gz ./target/release/profiling

heap-risczero:
	cd risczero && \
	heaptrack --output ../heaptrack.risczero.gz ./target/release/profiling

heap-powdr:
	cd powdr && \
	heaptrack --output ../heaptrack.powdr.gz ./target/release/profiling

heap-jolt:
	cd jolt && \
	heaptrack --output ../heaptrack.jolt.gz ./target/release/profiling
