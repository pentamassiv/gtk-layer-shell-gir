[![maintenance-status: passively-maintained (as of 2022-10-01)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2022--10--01%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk-layer-shell-gir/build_x86.yaml?branch=main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86.yaml)

gtk-layer-shell:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell.svg)](https://crates.io/crates/gtk-layer-shell)
[![docs.rs](https://docs.rs/gtk-layer-shell/badge.svg)](https://docs.rs/gtk-layer-shell)
[![dependency status](https://deps.rs/crate/gtk-layer-shell/0.5.1/status.svg)](https://deps.rs/crate/gtk-layer-shell/0.5.1)

# gtk-layer-shell
This is the safe wrapper for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell), automatically generated from its [.gir file](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gir-files/GtkLayerShell-0.1.gir). The unsafe bindings can be found [here](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell-sys). Unfortunately this crate is GTK3 only because upstream does not yet support GTK4 (https://github.com/wmww/gtk-layer-shell/issues/37).

## Usage
Have a look at the [simple example](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell/examples/simple-example.rs) to see how the bindings can be used. It works analogous to the original. You can use the features to select the version of gtk-layer-shell. Currently v0_6 is the default.

## Generating the wrapper
Generating the wrapper yourself is not necessary to be able to use it. You can just use the version published on crates.io. If you want to do it anyways, you need to start by [generating the unsafe bindings](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell-sys/README.md#generating-the-bindings). Follow the guide on how to do that and come back here.

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
gir -c Gir.toml --doc-target-path docs.md -m doc
cargo install rustdoc-stripper
rustdoc-stripper -s -n
rustdoc-stripper -g -o docs.md
cargo doc
```
Congratulations, you've done it :-)

If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Maintenance status
This crate is just a safe wrapper for the C library so it is feature complete and not actively worked on. There are Github Actions keeping the dependencies up-to-date. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
