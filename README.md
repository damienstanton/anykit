# anykit 🛠

[![Build Status](https://github.com/damienstanton/anykit/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/damienstanton/anykit/actions/workflows/rust.yml)

This is a workspace designed to be used as follows:

- A single codebase is updated in the `anykit` crate
- The `anykit` crate exposes APIs for just about everything, from asynchronouse web calls to running as a distributed actor system
- That crate contains an Azure function / CLI called `anyfunc`
- The `anykit` crate itseld cross-compiled into packages for other ecosystems:
  - `anykit-py`
  - `anykit-js`
  - `anykit-go`
  - `anykit-dotnet`

© 2022 Damien Stanton

See LICENSE for details.
