ecc bashreadline.bpf.c
cargo build 
sudo ecli run package.json &
sudo ./target/debug/curn --debug --command /bin/bash --mount ./ubuntu-fs --uid 0 --add ../lim:/tmp/lim --tool ../curn-tool