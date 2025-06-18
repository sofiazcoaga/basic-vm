# Basic LC3 Virtual Machine
This repository contains an MVP for a basic virtual machine implementation for LC3 architecture. It is a Rust version of [this tutorial](https://www.jmeiners.com/lc3-vm/#:lc3.c_2).

## What is a VM?
A VM is a program that acts like a computer. It simulates a CPU along with a few other hardware components, allowing it to perform arithmetic, read and write to memory, and interact with I/O devices, just like a physical computer. Most importantly, it can understand a machine language which you can use to program it.
(This paragraph was taken from [Justin Meiners' "Write your own virtual machine"](https://www.jmeiners.com/lc3-vm/#what-is-a-virtual-machine-)).

## What is LC-3?
LC-3 is an assembly language created for teaching purposes that implements a reduced set of instructions. The architecture includes:
- a set of 16 instructions.
- a memory with a 2^16 location address space.
- a word of 16 bits.

For further information on LC-3 check:
- [Architectural specifications](https://en.wikipedia.org/wiki/Little_Computer_3#Architectural_specification).
- [Instructions specifications](https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf).

## Requirements
- [Rust toolchain installed](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Make installed.

## Run a binary with the VM
In order to run a binary with the VM you can run the command
```make run path=<binary-path>```

This repository provides two example programs that are inside the `binary-examples` repository. In order to execute any of them you could use:

`make run path=./binary-examples/2048.obj`

or

`make run path=./binary-examples/rogue.obj`

The binaries were taken as examples from the following repositories:
- [lc3-rogue](https://github.com/justinmeiners/lc3-rogue).
- [lc3-2048](https://github.com/rpendleton/lc3-2048).

## Documentation
This repository contains full explanatory inline comments for the implementation. In order to see it in a friendlier way you can run
```make doc```
that will generate source code documentation and open it in the browser.

## Other useful commands
- `make build`: allows the user to build the VM.
- `make test`: allows the user to run the tests.
