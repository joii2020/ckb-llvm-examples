#include "ckb_consts.h"
#include "ckb_syscalls.h"

__attribute__((visibility("default"))) int dl_add(int a, int b) { return a + b; }

__attribute__((visibility("default"))) int dl_sub(int a, int b) { return a - b; }
