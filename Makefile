hello:
	echo hello
	
cargo:
	cargo build --release --bin mal_rust

step0_repl: cargo
	cp target/release/mal_rust step0_repl
	
clean:
	rm step*