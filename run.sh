sudo ./target/release/curn --command /bin/bash --mount ./ubuntu-fs --uid 0 --add ../lim/tests/:/tmp/lim/ --add ./tests/:/tmp/tests/ --tool ./curn-tool
