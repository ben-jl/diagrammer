# Diagrammer

## Getting Started

1. Install Rust and associated toolchains

    ```sh
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ...
    Rust is installed now. Great!
    ...

    $ source $HOME/.cargo/env

    $ rustup update
    info: syncing channel updates for 'stable-x86_64-unknown-linux-musl'
    info: checking for self-updates

    stable-x86_64-unknown-linux-musl unchanged - rustc 1.55.0 (c8dfcfe04 2021-09-06)

    info: cleaning up downloads & tmp directories

    $ su -c "apk add gcc"
    (1/9) Installing binutils (2.35.2-r1)
    (2/9) Installing libgomp (10.2.1_pre1-r3)
    (3/9) Installing libatomic (10.2.1_pre1-r3)
    (4/9) Installing libgphobos (10.2.1_pre1-r3)
    (5/9) Installing gmp (6.2.1-r0)
    (6/9) Installing isl22 (0.22-r0)
    (7/9) Installing mpfr4 (4.1.0-r0)
    (8/9) Installing mpc1 (1.2.0-r0)
    (9/9) Installing gcc (10.2.1_pre1-r3)
    Executing busybox-1.32.1-r5.trigger
    OK: 132 MiB in 35 packages

    $ su -c "apk add libc-dev=0.7.2-r3"
    (1/3) Upgrading musl (1.2.2-r0 -> 1.2.2-r1)
    (2/3) Installing musl-dev (1.2.2-r1)
    (3/3) Installing libc-dev (0.7.2-r3)
    OK: 142 MiB in 37 packages

    $ cargo install cargo-watch
    ...
    Installing /home/blevalley/.cargo/bin/cargo-watch
    Installed package `cargo-watch v8.1.1` (executable `cargo-watch`)

    ```
