# anykit 🛠

[![Build Status](https://github.com/damienstanton/anykit/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/damienstanton/anykit/actions/workflows/rust.yml)

This is a workspace designed to be used as follows:

- A single codebase is updated in the `anykit` crate
- The `anykit` crate exposes APIs for just about everything, from asynchronouse web calls to running as a distributed actor system
- That crate contains an Azure function / CLI called `anyfunc`
- The `anykit` crate is cross-compiled into its two sister crates, `anykit-py` and `anykit-js`, for use in those ecosystems.

© 2022 Damien Stanton

See LICENSE for details.
