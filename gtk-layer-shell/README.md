[![Crate](https://img.shields.io/crates/v/gtk-layer-shell.svg)](https://crates.io/crates/gtk-layer-shell)
[![dependency status](https://deps.rs/crate/gtk-layer-shell/0.3.1/status.svg)](https://deps.rs/crate/gtk-layer-shell/0.3.1)
[![Build_x86](https://img.shields.io/github/workflow/status/pentamassiv/gtk-layer-shell-gir/Build_x86/main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86_64.yaml)
[![Build_aarch64](https://img.shields.io/github/workflow/status/pentamassiv/gtk-layer-shell-gir/Build_aarch64/main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_aarch64.yaml)

# gtk-layer-shell
This is the safe wrapper for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell), automatically generated from its [.gir file](../gir-files/GtkLayerShell-0.1.gir). The unsafe bindings can be found [here](../gtk-layer-shell-sys).

## Usage
Have a look at the [simple example](examples/simple-example.rs) to see how the bindings can be used. It works analogous to the original. You can use the features to select the version of gtk-layer-shell. Currently v0_6 is the default.

## Generating the wrapper
Generating the wrapper yourself is not necessary to be able to use it. You can just use the version published on crates.io. If you want to do it anyways, you need to start by [generating the unsafe bindings](../gtk-layer-shell-sys/README.md#generating-the-bindings). Follow the guide on how to do that and come back here.

Now that you have generated the bindings you will want to generate the safe wrapper.
```bash
cd gtk-layer-shell
gir -o .
cargo build
cargo test
```
There should not have been any errors.
To make sure everything you need was created, run the following command.
```bash
gir -o . -m not_bound
```
There should not be any output to this command. Let me know if there is so I can fix it. 
In order to build the documentation, you have to run
```
gir -c Gir.toml -d ../gir-files --doc-target-path docs.md -m doc
cargo install rustdoc-stripper
rustdoc-stripper -s -n
rustdoc-stripper -g -o docs.md
cargo doc
```
Congratulations, you've done it :-)

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
