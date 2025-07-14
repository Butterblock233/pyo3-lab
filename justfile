set windows-shell := ["pwsh", "-nologo","-noprofile","-command"]

# Build and install the package, including generating stubs
build_package: gen_stubs
	@echo "Building pypi package"
	uv pip install -e ./rust

build_rust:gen_stubs
	@echo "Building rust package"
	uvx maturin develop --uv

# Generate .pyi stub files
gen_stubs:
	@echo "Generating stubs"
	@cd rust && cargo run --bin stub_gen

# Clean the rust project
clean_rust:
    cd rust && cargo clean
