# PyO3 Lab
This project shows how to use PyO3 and maturin to write python module in rust

# Getting Started


This project use `uv` to manage python codes. Run `uv sync` to setup python environment.

To build python module, just simply run
```
just build_package
```

This will build and install the module to your env.

Then you can run `ux pip list` to check installed packages.