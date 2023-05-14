# HW 6 "adapter"

## Requirements

* rustup
* [cargo make]( https://github.com/sagiegurari/cargo-make )
* [cross-rs]( https://github.com/cross-rs/cross )

## Usage

- Use ``cargo make --profile production assembly_linux`` to assembly binaries for Linux
- Use `` cargo make --profile production oassembly_windows`` to asembly binaries for Windows
- Use ``cargo build -Z unstable-options`` to build executables with cargo

* binaries can be found in ``binaries`` directory

## How to use

- You can create file filled with two matrixes by running ``./prog_creator -o matrixes``
- You can sum two matrixes and write result to a file by rinning ``./prog_reader -i matrixes -o sum_matrix``
- Test ADAPTER example by running ``./prog_adapter -m matrix_files -o sum_file`` to create file ``matrix_files`` with matrixes and write sum of matrixes to ``sum_file``
- Verify result ru running usual program ``./prog_reader -i matrix_files -o sum_matrix``


## UML diagram

![](https://github.com/Kerosin3/Programming-architecture-and-patterns/blob/cargo-make/homework6_adapter_independent/pics/hw6.jpg)

