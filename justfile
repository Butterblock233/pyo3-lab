set windows-shell := ["pwsh", "-nologo","-noprofile","-command"]

# Build and install the package, including generating stubs
build_package: gen_stubs
	uv pip install -e ./rust

build_rust:gen_stubs
	uvx maturin develop --uv

# Generate .pyi stub files
gen_stubs:
	cd rust && cargo run --bin stub_gen

# Clean the rust project
clean_rust:
    cd rust && cargo clean
