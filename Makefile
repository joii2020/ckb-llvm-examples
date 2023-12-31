CC := clang
LD := ld.lld
OBJCOPY := llvm-objcopy

CFLAGS := \
		--target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-O3 -g -Wall -Werror \
		-DCKB_NO_ENTRY_GP \
		-fno-builtin-printf -fno-builtin-memcmp \
		-nostdinc -nostdlib \
		-fvisibility=hidden \
		-fdata-sections -ffunction-sections \
		-I c \
		-I deps/ckb-c-stdlib \
		-I deps/ckb-c-stdlib/libc \
		-I deps/ckb-c-stdlib/molecule

LDFLAGS := -Wl,-static -Wl,--gc-sections

all: \
	build/simple \
	build/dl \
	build/dl_demo \
	build/dl_demo_tx

run-all: run-simple

build/simple: c/simple.c
	mkdir -p build
	$(CC) $(CFLAGS) -o $@ $<
	cp $@ $@.debug
	$(OBJCOPY) --strip-debug --strip-all $@

run-simple: build/simple
	ckb-debugger --bin build/simple

build/dl: c/dl.c
	mkdir -p build
	$(CC) $(CFLAGS) -fPIC -c -o $@.o $<
	$(LD) --shared --gc-sections --dynamic-list c/dl.syms -o $@ $@.o
	cp $@ $@.debug
	$(OBJCOPY) --strip-debug --strip-all $@

build/dl_demo: c/dl_demo/dl_demo.c
	$(CC) $(CFLAGS) -o $@ $<
	cp $@ $@.debug
	$(OBJCOPY) --strip-debug --strip-all $@

build/dl_demo_tx: build/dl build/dl_demo
	cd c/dl_demo/generate_tx && cargo run > ../../../build/tx.json

run-dl-demo: build/dl build/dl_demo build/dl_demo_tx
	ckb-debugger --tx-file=build/tx.json -s lock --max-cycles 10000000000

clean:
	rm -rf build/*
