[![maintenance-status: passively-maintained (as of 2023-08-03)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2023--08--03%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk-layer-shell-gir/build_x86.yaml?branch=main)](https://github.com/pentamassiv/gtk-layer-shell-gir/actions/workflows/build_x86.yaml)

gtk-layer-shell:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell.svg)](https://crates.io/crates/gtk-layer-shell)
[![docs.rs](https://docs.rs/gtk-layer-shell/badge.svg)](https://docs.rs/gtk-layer-shell)
[![dependency status](https://deps.rs/crate/gtk-layer-shell/0.8.1/status.svg)](https://deps.rs/crate/gtk-layer-shell/0.8.1)

gtk-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk-layer-shell-sys.svg)](https://crates.io/crates/gtk-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk-layer-shell-sys/badge.svg)](https://docs.rs/gtk-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk-layer-shell-sys/0.7.1/status.svg)](https://deps.rs/crate/gtk-layer-shell-sys/0.7.1)


# gtk-layer-shell
This is the safe wrapper for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell), automatically generated from its [.gir file](GtkLayerShell-0.1.gir). For details on how to use it or how to generate the code yourself, have a look at the crate's [README](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell/README.md). The unsafe bindings can be found in this [folder](https://github.com/pentamassiv/gtk-layer-shell-gir/tree/main/gtk-layer-shell-sys). These crates are GTK3 only. Use [gtk4-layer-shell-sys](https://crates.io/crates/gtk4-layer-shell-sys) or [gtk4-layer-shell](https://crates.io/crates/gtk4-layer-shell) if you need to use it with GTK4.

## Maintenance status
The Rust bindings for GTK3 are no longer maintained and the repo has been archived. If you are starting a new project, you should use GTK4 and thus gtk4-layer-shell right from the start. 

The gtk-layer-shell crates are just wrappers for the C library so it is feature complete and not actively worked on. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
