clean:
	find . -type f -name "Cargo.toml" -execdir sh -c 'cargo clean' \;
