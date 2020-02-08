CXX := gcc
# CFLAGS := -Iinclude
# LDFLAGS := -Llibs/x64-linux-gcc41 -lrtmp -lfmpmem -lssl -lexpat

SRC := $(wildcard *.c)
OBJ := $(patsubst %.c,%.o,$(SRC)) libmylib.a

test: build-test
	./test
		
build-test: $(OBJ)
	$(CXX) $(CPPFLAGS) libmylib.a -o test $^ $(LDFLAGS)

test-all: cargo-test build-test
	./test

clean:
	rm -f $(OBJ) test

cargo-clean:
	(cd mylib && cargo clean)

cargo-build:
	(cd mylib && cargo build)

cargo-test:
	(cd mylib && cargo test)

libmylib.a: cargo-build
	cp mylib/target/debug/libmylib.a .

%.o: %.c
	$(CXX) $(CFLAGS) -o $@ -c $<

