TARGET = qemu
#TARGET = stm32f429zi

BINARY = target/$(TARGET)/release/zpm

all:
	xargo build --target=$(TARGET) --verbose --release
	arm-none-eabi-readelf -a $(BINARY) > target/readelf.txt
	arm-none-eabi-objdump -Cd $(BINARY) > target/objdump.txt
	arm-none-eabi-size $(BINARY)

clean:
	cargo clean

run:
	qemu-system-arm -machine lm3s6965evb -gdb tcp::3333 -S -nographic -monitor null -serial null -semihosting -kernel $(BINARY) &
	arm-none-eabi-gdb $(BINARY)
