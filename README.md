# Basic LC3 Virtual Machine

This repository contains an MVP for a basic virtual machine implementation for LC3 architecture. It is a Rust version of [this tutorial](https://www.jmeiners.com/lc3-vm/#:lc3.c_2).

## Requirements
- [Rust toolchain installed](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Make installed.

## How to interact with the VM
The VM contains a Makefile that makes it easier to interact with. The available commands are:
- `make run path=<some-path>`: allows the user to execute an LC-3 program with the virtual machine. This repository provides two example programs that are inside the `binary-examples` repository. In order to execute any of them you could use:
    - `make run path=./binary-examples/2048.obj` // This binary belongs [here](https://github.com/rpendleton/lc3-2048).
    - `make run path=./binary-examples/rogue.obj` // This binary belongs [here](https://github.com/justinmeiners/lc3-rogue).

- `make build`: allows the user to build the VM.
- `make test`: allows the user to run the tests.
- `make doc`: generates and open source code documentation in the browser. 
