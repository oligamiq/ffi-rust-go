#include <stdio.h>
void ffi_gophernize_num( );

void ffi_ffi_gophernize_num( ) {
  printf("(((((((())))))))");
  ffi_gophernize_num( );
  printf("{{{{{{{{}}}}}}}}");
}
// gcc -c ffi_go_rust.c -L. -lnosql_db_sdk_ffi
// clang -c ffi_go_rust.c -L. -lnosql_db_sdk_ffi
// ar rcs libffi_go_rust.lib ffi_go_rust.o
