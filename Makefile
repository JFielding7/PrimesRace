c:
	gcc -O3 C/main.c -o C/c_primes
	./C/c_primes

rust:
	cargo run --release --manifest-path Rust/Cargo.toml