#include <stdio.h>

void A( ) {
  int num = 5;    //静的ライブラリのここだけは違う
  printf("inside A, num = %d\n", num);
}

// gcc -c makelib.c
// ar rcs libA.lib makelib.o
