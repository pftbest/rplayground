TARGET=stm32f429zi

all:
	xargo build --target=$(TARGET) --verbose --release
	md5sum target/$(TARGET)/release/zpm
clean:
	cargo clean
