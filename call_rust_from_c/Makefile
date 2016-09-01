ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread
endif

all: target/call_rust
	target/call_rust

target:
	mkdir -p $@

target/call_rust: target/main.o target/debug/libport.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libport.a: src/port.rs Cargo.toml
	cargo build

target/main.o: c.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
