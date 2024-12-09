[![maintenance-status: obsolete](https://img.shields.io/badge/maintenance--status-obsolete-red)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk-layer-shell-gir/build_x86.yaml?branch=main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86.yaml)

gtk-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell-sys.svg)](https://crates.io/crates/gtk-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk-layer-shell-sys/badge.svg)](https://docs.rs/gtk-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk-layer-shell-sys/0.7.2/status.svg)](https://deps.rs/crate/gtk-layer-shell-sys/0.7.2)

# This project is UNMAINTAINED. Please use GTK4 and take a look at [gtk4-layer-shell](https://crates.io/crates/gtk4-layer-shell) instead!

# gtk-layer-shell-sys
These are the unsafe FFI bindings for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell). They were automatically generated from its [.gir file](../GtkLayerShell-0.1.gir). This crate is GTK3 only. Use [gtk4-layer-shell](https://crates.io/crates/gtk4-layer-shell) if you need to use it with GTK4.

## Usage
These are the unsafe bindings. You most likely want to use the safe [wrapper](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell). If you are sure you want the unsafe bindings, you can use the features to select the version of gtk-layer-shell. Default currently is v0_6.

## Generating the bindings
Generating the bindings yourself is not necessary to be able to use it. If you want to do it anyways, here are the steps you can follow to generate the bindings yourself.

You need to have Rust, Gtk3 and gtk-layer-shell installed installed. Clone the repository AND the submodules "gir" and "gir-files".
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

There should not have been any errors. You should now continue and generate the [safe wrapper](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell/README.md#generating-the-wrapper).
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Maintenance status
The Rust bindings for GTK3 are no longer maintained and the repo has been archived. If you are starting a new project, you should use GTK4 and thus gtk4-layer-shell right from the start.

This crate is just an unsafe wrapper for the C library so it is feature complete and not actively worked on. There are Github Actions keeping the dependencies up-to-date. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
