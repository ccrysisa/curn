all: package.json
	cargo build --release

debug: package.json
	cargo build

package.json: ecc
	./ecc snoop.bpf.c snoop.bpf.h

clean:
	@cargo clean
	@rm -rf *.bpf.o *.json

logs:
	@rm -rf logs/*

.PHONY: all debug clean logs