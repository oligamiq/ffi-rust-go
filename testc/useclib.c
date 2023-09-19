#include <stdio.h>

void A( );

int main( ) {
  printf("$$");

  A( );
}

// gcc useclib.c -o useclib -L. -lA
