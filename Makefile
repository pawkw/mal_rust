hello:
	echo hello
	
cargo:
	cargo build --release --bin mal_rust

step0_repl: cargo
	cp target/release/mal_rust step0_repl

step1_read_print: cargo
	cp target/release/mal_rust step1_read_print
	
clean:
	rm step*