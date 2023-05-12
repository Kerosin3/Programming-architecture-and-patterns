# HW 5 "abstract factory"

## Requirements

* rustup
* [cargo make]( https://github.com/sagiegurari/cargo-make )
* [cross-rs]( https://github.com/cross-rs/cross )

## Usage

- Use ``cargo make --profile production assembly_linux`` to assembly binaries for Linux
- Use `` cargo make --profile production assembly_windows`` to asembly binaries for Windows
- Use ``cargo make test_all`` or ``cargo test`` **to run tests**

## How to use (linux example)
1. use ``sorhing -h`` to get help
2. Run binary to produce sample file with random data(i32) by running `` ./sorting-app -w write-random --output-filename test_out``
3. Read written numbers ``./sorting-app -w read-file --input-filename test_out -d true``
4. Sort with specified method [possible values: bubble, merge, quick] ``./sorting-app -m merge -w operating --input-filename test_out --output-filename  sorted``
5. Verify that numbers were sorted correctly by running ``./sorting-app -w read-file --input-filename sorted -d true``
6. Verify method signature ``tail --byte 30 sorted``

## UML diagram

![](https://github.com/Kerosin3/Programming-architecture-and-patterns/blob/cargo-make/homework5_abstract_factory/pics/hw5.jpg)

