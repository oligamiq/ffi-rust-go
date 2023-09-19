#include <stdio.h>

void ffi_gophernize_num( );

int main( ) {
  printf("$$");

  ffi_gophernize_num( );
}

// gcc -c uselib.c -L. -lnosql_db_sdk_ffi
