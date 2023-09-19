package main

import (
	"C"
)

//export ffi_go_print
func ffi_go_print() {
	println(":print on golang")
}

func main() {}
