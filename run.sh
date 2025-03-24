ecc bashreadline.bpf.c
cargo build --release
sudo ./target/release/curn --command /bin/bash --mount ./ubuntu-fs --uid 0