
BIN := target/debug/rust

$(BIN): src/main.rs
	cargo build

memtest: $(BIN)
	echo "ok\nfarts\nstuff\nzebra\njunk\naaa\nzzz" | target/debug/rust
