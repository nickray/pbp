run-examples:
	cargo run --features dalek --example print
	cargo run --example read-sig < examples/props/sig.txt
	cargo run --features dalek --example round-trip
	cargo run --features dalek --example verify-sig
