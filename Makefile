all:
	./ecc bashreadline.bpf.c
	cargo build
	cargo build --release

clean:
	cargo clean
	rm -rf *.bpf.o *.json