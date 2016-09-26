all:
	xargo build --target=qemu --verbose --release
	md5 target/qemu/release/zpm
clean:
	cargo clean
