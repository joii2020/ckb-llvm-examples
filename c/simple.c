
#define CKB_C_STDLIB_PRINTF
#include <stdio.h>

#ifndef BUILD_NATIVE
#include "ckb_syscalls.h"
#endif //BUILD_NATIVE

void test_aaaa() {
  printf("-- test_aaaa --\n");
}

int test_aaa() {
  printf("Test AAA\n");
  int a = 2;
  test_aaaa();
  a += 10;
  printf("TestAAA Exit, a: %d\n", a);
  return a;
}

int main() {
  printf("Hello Wrold!\n");
  test_aaa();
  printf("---- Exit ----\n");
  return 0;
}
