[![maintenance-status: passively-maintained (as of 2022-10-01)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2022--10--01%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk-layer-shell-gir/build_x86.yaml?branch=main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86.yaml)

gtk-layer-shell:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell.svg)](https://crates.io/crates/gtk-layer-shell)
[![docs.rs](https://docs.rs/gtk-layer-shell/badge.svg)](https://docs.rs/gtk-layer-shell)
[![dependency status](https://deps.rs/crate/gtk-layer-shell/0.5.1/status.svg)](https://deps.rs/crate/gtk-layer-shell/0.5.1)

gtk-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell-sys.svg)](https://crates.io/crates/gtk-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk-layer-shell-sys/badge.svg)](https://docs.rs/gtk-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk-layer-shell-sys/0.6.0/status.svg)](https://deps.rs/crate/gtk-layer-shell-sys/0.6.0)


# gtk-layer-shell
This is the safe wrapper for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell), automatically generated from its [.gir file](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gir-files/GtkLayerShell-0.1.gir). For details on how to use it or how to generate the code yourself, have a look at the crate's [README](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell/README.md). The unsafe bindings can be found in this [folder](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell-sys). Unfortunately this crate is GTK3 only because upstream does not yet support GTK4 (https://github.com/wmww/gtk-layer-shell/issues/37).

## Maintenance status
These crates are just wrappers for the C library so it is feature complete and not actively worked on. There are Github Actions keeping the dependencies up-to-date. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
