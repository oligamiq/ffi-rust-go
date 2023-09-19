LIST_TARGET_DIR := golib/target/list
list:
	@mkdir $(LIST_TARGET_DIR)
	@cd golib && go build -buildmode=c-archive -o target/list/list.a main.go
	@cd $(LIST_TARGET_DIR) && ar -x list.a
	@find $(LIST_TARGET_DIR) -type f -name '*.o' ! -name 'go.o' -exec nm {} \; | grep -v 'cgo' | awk '$$2 == "T" {print $$3}' | grep -v -e "crosscall_amd64" -e "fatalf"
	@cd $(LIST_TARGET_DIR) && cd .. && rm -rf list
