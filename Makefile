TARGET=stm32f429zi

ifeq (, $(shell which md5sum))
  MD5SUM = md5 -r
else
  MD5SUM = md5sum
endif

all:
	xargo build --target=$(TARGET) --verbose --release
	$(MD5SUM) target/$(TARGET)/release/zpm

clean:
	cargo clean
