.PHONY: run
run: loader library.so
	./loader

.PHONY: clean
clean:
	-rm -r loader library.so target

loader: src/main.rs
	cargo build
	cp ./target/debug/libloading ./loader

library.so: library.c
	gcc -shared -Wall -fpic -o $@ $^
