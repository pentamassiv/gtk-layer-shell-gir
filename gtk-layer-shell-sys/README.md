[![Crate](https://img.shields.io/crates/v/gtk-layer-shell-sys.svg)](https://crates.io/crates/gtk-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk-layer-shell-sys/badge.svg)](https://docs.rs/gtk-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk-layer-shell-sys/0.4.0/status.svg)](https://deps.rs/crate/gtk-layer-shell-sys/0.4.0)
[![Build_x86](https://img.shields.io/github/workflow/status/pentamassiv/gtk-layer-shell-gir/Build_x86/main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86_64.yaml)
[![Build_aarch64](https://img.shields.io/github/workflow/status/pentamassiv/gtk-layer-shell-gir/Build_aarch64/main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_aarch64.yaml)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

# gtk-layer-shell-sys
These are the unsafe FFI bindings for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell). They were automatically generated from its [.gir file](../gir-files/GtkLayerShell-0.1.gir). Unfortunately this crate is GTK3 only because upstream does not yet support GTK4 (https://github.com/wmww/gtk-layer-shell/issues/37).

## Usage
These are the unsafe bindings. You most likely want to use the safe [wrapper](../gtk-layer-shell). If you are sure you want the unsafe bindings, you can use the features to select the version of gtk-layer-shell. Default currently is v0_6.

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
If you regenerate the binding, because you have a new version of the GtkLayerShell gir file, copy it into the [gir files](../gir-files) folder.
Now you can generate, build and test the bindings.
```bash
cd gtk-layer-shell-sys
gir -o .       # Regenerate the bindings
cargo build    # Build the created bindings
cargo test     # Test the created bindings
cd ..
```

There should not have been any errors. You should now continue and generate the [safe wrapper](../gtk-layer-shell/README.md#generating-the-wrapper).
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Changelog

### 0.3.0
If you come from a previous version, there is no need for you to do anything if you use version v0_6 of the C library. If you use an older version, you need to deactivate the default features and use the feature matching the version you are using.
- Docs are properly added and it builds on docs.rs
- No more manual edits to generated files of this crate needed
- Updated dependencies
- gtk_layer_get_zwlr_layer_surface_v1 function is no longer ignored but properly generated and can be used in the sys and wrapper crate
- The default version of the C library is now v0_6. Turn off the default feature and select the feature you need, if you use an older version
- Added workflows to automate updates so I don't need to actively do much :-)

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
