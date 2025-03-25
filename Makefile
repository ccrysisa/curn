all:
	./ecc bashreadline.bpf.c
	cargo build --release

debug:
	./ecc bashreadline.bpf.c
	cargo build

clean:
	@cargo clean
	@rm -rf *.bpf.o *.json

.PHONY: all debug clean