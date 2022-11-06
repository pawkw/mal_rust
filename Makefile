hello:
	echo hello
	
step0_repl:
	cargo build --release --bin mal_rust
	cp target/release/mal_rust step0_repl
	
clean:
	rm step*