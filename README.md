# Advanced Computer Architecture Coursework

This repository will contain all resources to do with my Advanced Computer
Architecture coursework.

In this coursework I will build an instruction set architecture as well as a
super-scalar processor simulator for it. I will be coding this in `rust`, which
is a new language for me so excuse any novice tendancies!

_Disclaimer! This will likely have bugs in it, and should not be used for any
important work!_

## Getting Started:

As this project is written in Rust, you will need to install the Rust
toolchain.

Fortunately there exists a script (which I have bundled in the project) that
will install this to user-space on Linux based machines. In order to build the
project run:

```bash
$ make rust-install   # Installs Rust
$ source ~/.cargo/env # Loads PATH
$ make run            # Builds and runs
```

Or run your own command with (refer below for arguments):

```bash
$ ./target/release/daybreak <args>
```

Once in the simulator:

  - `Left` and `Right` arrow keys allow you to navigate states of the
     simulator, forwards and backwards in time.
  - The `Space` bar will pause and un-pause the simulation.
  - `Esc` or `Q` to quit.

_Note: Backwards in time is limited to the last 250 entries, and un-pausing
is only possible from the latest state._

And finally, should you wish to clean up the project and remove everything installed:

```bash
$ make clean       # Cleans project
$ make rust-remove # Removes Rust
```

## Runtime Argumnets:

The most up to date information should from `./daybreak --help

```
Project Daybreak 0.1.0
Anthony W. <a.wharton.2015@bristol.ac.uk>
A superscalar, out of order, riscv32im simulator.

USAGE:
    daybreak [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -h, --help            Prints help information
    -r, --return-stack    Enables the Return Address Stack.
    -V, --version         Prints version information

OPTIONS:
        --alu <N>
            Sets the number of Arithmetic Logic Units. [default: 1]

        --blu <N>
            Sets the number of Branch Logic Units. [default: 1]

    -b, --branch-prediction <branch-prediction>
            Sets the branch prediction mode. [default: twobit]  [possible values: off, onebit,
            twobit, twolevel]
    -i, --issue-limit <N>
            Sets a limit to the number of instructions issued and committed per cycle. Setting this
            to 0 is interpreted as the number of execute units. [default: 1]
        --mcu <N>
            Sets the number of Memory Control Units. [default: 1]

    -n, --n-way <N>
            Sets the 'n-way-ness' of the fetch, decode and commit stages. [default: 1]

        --rob <N>
            Sets the number of entries in the reorder buffer. [default: 32]

        --rsv <N>
            Sets the number of entries in the reservation station. [default: 16]


ARGS:
    <FILE>    Specifies a path to elf file to execute in the simulator.
```

## Compile-able Options:

Some options are not configurable from the command line, these are:

  - Execution lengths/Per-instruction blocking is defined at
    `./src/simulator/execute.rs:80`
  - Execute Unit pipeline length is defined at `./src/simulator/state.rs:87`

