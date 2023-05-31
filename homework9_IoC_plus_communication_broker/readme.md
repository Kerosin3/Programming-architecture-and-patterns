# HW 9 "Inversion of Control in Agent model programming (communication broker)"

## Requirements

* rustup
* [cargo make]( https://github.com/sagiegurari/cargo-make )
* [cross-rs]( https://github.com/cross-rs/cross )
* [rumqtt] ( https://github.com/bytebeamio/rumqtt ) MTQQ Broker

## Usage

- Use ``cargo make test_all`` or ``cargo test -- --nocapture`` to run test
- Use ``cargo make --profile production assembly_linux`` to assembly binaries for Linux
- Use ``cargo make --profile production assembly_windows`` to asembly binaries for Windows
- Use ``cargo build -Z unstable-options`` to build executables with cargo

* binaries can be found in ``binaries`` directory

## Run example
1. run MTQQ Broker on 1883 port !!!!
2. run example main bridge processor agent ``agent_processor``
3. run gameserver agent ``agent_gameserver``
4. run example agent player ``agent_sender``
5. examime ``agent_gameserver`` output to receiver data containitg command with prefix ``Srv``
