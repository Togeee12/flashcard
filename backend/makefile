OUTPUT_FOLDER := ./builds
TARGET_TRIPLE := x86_64-ubuntu_22.04-linux-gnu

.PHONY: build clean

build:
	@mkdir $(OUTPUT_FOLDER)
	cargo build --release
	@mv target/release/backend $(OUTPUT_FOLDER)/backend-$(TARGET_TRIPLE)
	cargo build --release --features demo
	@mv target/release/backend $(OUTPUT_FOLDER)/backend-$(TARGET_TRIPLE)-demo

clean:
	@rm -rf $(OUTPUT_FOLDER)
	@cargo clean