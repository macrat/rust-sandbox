.PHONY: run
run: loader
	LD_LIBRARY_PATH=. ./loader

.PHONY: clean
clean:
	-rm -r libcdylib.so loader target

loader: loader.c libcdylib.so
	gcc -o $@ -L. -lcdylib $^

libcdylib.so: src/lib.rs
	cargo build
	cp target/debug/libcdylib.so .
