[![Crate](https://img.shields.io/crates/v/gtk-layer-shell-sys.svg)](https://crates.io/crates/gtk-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk-layer-shell-sys/0.2.6/status.svg)](https://deps.rs/crate/gtk-layer-shell-sys/0.2.6)
[![Build_x86](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86_64.yaml/badge.svg)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86_64.yaml)
[![Build_aarch64](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_aarch64.yaml/badge.svg)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_aarch64.yaml)

# gtk-layer-shell-sys
These are the unsafe FFI bindings for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell). They were automatically generated from its [.gir file](../../gir-files/GtkLayerShell-0.1.gir).

## Usage
These are the unsafe bindings. You most likely want to use the safe [wrapper](..).

## Generating the bindings
Generating the bindings yourself is not necessary to be able to use it. If you want to do it anyways, here are the steps you can follow to generate the bindings yourself.

You need to have Rust, and Gtk3 installed. Clone the repository AND the submodule "gir".
```bash
git clone --recurse-submodules -j8 https://github.com/pentamassiv/gtk-layer-shell-gir.git
cd ./gtk-layer-shell-gir
```
Then you need to install gir.
```bash
cd gir
cargo install --path .
cd ..
```
If you regenerate the binding, because you have a new version of the GtkLayerShell gir file, copy it into the [gir files](../../gir-files) folder.
Now you can generate, build and test the bindings.
```bash
cd gtk-layer-shell/gtk-layer-shell-sys
gir -o .                       # Regenerate the bindings
cargo build --features v0_6    # Build the created bindings
cargo test --features v0_6     # Test the created bindings
cd ..
```

There should not have been any errors. You should now continue and generate the [safe wrapper](../README.md#generating-the-wrapper).
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
