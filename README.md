# mipz-practice-1

CLI application that implements Euro Diffusion.

It is supported on all mainstream platforms `Windows/Linux/MacOS`.

To build the application you must have [rust toolchain installed](https://www.rust-lang.org/tools/install):

```bash
$ cargo --version
cargo 1.60.0 (d1fd9fe 2022-03-01)
```

In oder to compule the CLI binary run the following command:

```bash
cargo build

# Or compile it with optimizations in release mode

cargo build --release
```

To run the complied CLI application

```bash
./target/debug/euro-diffusion --help

# Or if the binary was compiled in release mode
./target/release/euro-diffusion --help
```
