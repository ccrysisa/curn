TEST_PATH=tests
TESTS=$(TEST_PATH)/test_mem $(TEST_PATH)/test_fd

all: package.json $(TESTS)
	cargo build --release

debug: package.json
	cargo build

package.json: ecc
	./ecc snoop.bpf.c snoop.bpf.h

$(TEST_PATH)/%: $(TEST_PATH)/%.c
	$(CC) $< -o $@

clean:
	@cargo clean
	@rm -rf *.bpf.o *.json
	@rm -rf $(TESTS)

clean-logs:
	@rm -rf logs/*

.PHONY: all debug clean logs tests