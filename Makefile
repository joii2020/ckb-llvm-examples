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
		-I deps/ckb-c-stdlib/libc

LDFLAGS := -Wl,-static -Wl,--gc-sections

all: simple

run-all: run-simple

simple: c/simple.c
	$(CC) $(CFLAGS) -o $@ $<
	cp $@ $@.debug
	$(OBJCOPY) --strip-debug --strip-all $@

run-simple: simple
	ckb-debugger --bin simple

clean:
	rm -rf simple simple.debug
