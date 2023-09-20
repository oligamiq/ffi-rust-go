// #include <stdio.h>

void ffi_go_print( );

int main( ) {
  // printf("$$");

  ffi_go_print( );
}

// gcc uselib.c -o uselib -L. -lffi_go_print