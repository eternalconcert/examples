#include <stdio.h>
#include <sys/io.h>

extern void rust_call();

int main() {
  ioperm(0x378, 1, 1);
  outb(0, 0x378);
  printf("C!\n");
  rust_call();
}
