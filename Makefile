TARGET=qemu
#TARGET=stm32f429zi

all:
	xargo build --target=$(TARGET) --verbose --release
	arm-none-eabi-readelf -a target/$(TARGET)/release/zpm > target/readelf.txt
	arm-none-eabi-objdump -Cd target/$(TARGET)/release/zpm > target/objdump.txt
	arm-none-eabi-size target/$(TARGET)/release/zpm

clean:
	cargo clean
