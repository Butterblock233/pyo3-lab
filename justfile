set windows-shell := ["pwsh", "-nologo","-noprofile","-command"]
build_rust:
	uvx maturin develop --uv

clean_rust:
    cd rust && cargo clean
