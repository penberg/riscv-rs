# riscv-rs

This is a RISC-V emulator written in Rust.
The goal of this project is to provide a platform for experimenting with extensions to the RISC-V instruction set.

## Roadmap

* [x] ELF support with `mmap()`
* [ ] RV32I instruction set support (_in progress_)

## Getting Started

You can run RISC-V ELF executables with `riscv-rs` as follows:

```
cargo run <program>
```

For example, download the RISC-V compliance tests and build them:

```
git clone https://github.com/riscv/riscv-compliance
cd riscv-compliance
$ RISCV_PREFIX=riscv64-linux-gnu- make
```

You can then run a test cases as follows:

```
$ cargo run riscv-compliance/work/rv32i/I-ADD-01.elf
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
