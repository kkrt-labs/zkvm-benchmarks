bench-all:
	#make bench-jolt
	make bench-sp1
	make bench-risczero
	make bench-zkm
	make bench-powdr
	make bench-openvm

bench-jolt:
	cd jolt && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1:
	make build-sp1
	cd sp1 && RUSTFLAGS="-C target-cpu=native" cargo run --release

bench-sp1-turbo:
	cd sp1-turbo/sha2/script && RUSTFLAGS="-C target-cpu=native" cargo run --release -- --prove

bench-zkm:
	make build-zkm
	cd zkm && RUSTFLAGS="-C target-cpu=native" cargo run --release

build-sp1:
	cd sp1/fibonacci && cargo prove build
	cd sp1/sha2-chain && cargo prove build
	cd sp1/sha3-chain && cargo prove build
	cd sp1/sha2 && cargo prove build
	cd sp1/sha3 && cargo prove build
	cd sp1/bigmem && cargo prove build

bench-risczero:
	cd risczero && RUSTFLAGS="-C target-cpu=native" cargo run --release -- fibonacci

bench-risczero-gpu:
	cd risczero && cargo run --release -F cuda

build-zkm:
	cd zkm/fibonacci && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha2 && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha3 && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/bigmem && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha2-chain && cargo build --target=mips-unknown-linux-musl --release
	cd zkm/sha3-chain && cargo build --target=mips-unknown-linux-musl --release

bench-powdr:
	cd powdr && RUSTFLAGS='-C target-cpu=native' cargo run --release

bench-openvm:
	cd openvm && \
	cargo openvm build && \
	cargo openvm keygen && \
	OPENVM_FAST_TEST=1 cargo openvm prove app --input "0x0A00000000000000"
