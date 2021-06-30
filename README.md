# scotch-rs

Idiomatic bindings to the [Scotch] partitioner.

## Building

Bindings to Scotch are made on the fly.  You'll need Scotch's header files and
shared libraries in order to build these bindings.  If those are in non-standard
locations, please use the following commands:

    export SCOTCHDIR=path/to/your/scotch/installation
    export CPATH="$SCOTCHDIR/include"
    export RUSTFLAGS="-L$SCOTCHDIR/lib"

The environment variable `$SCOTCHDIR` must point to a directory containing a
`lib/` and a `include/` directory containing the shared libraries and the
headers of Scotch, respectively.

Once these variables are set, you can build the bindings with `cargo build`.

### Build the documentation

If your Scotch installation lies in a non-standard path, you will need to set
the `RUSTDOCFLAGS` environment variable to build the documentation:

    export RUSTDOCFLAGS="-L$SCOTCHDIR/lib"

Then you can call `cargo doc --no-deps --open`.

## License

This program is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).  See `LICENSE-APACHE` and `LICENSE-MIT` for
details.

Scotch is licensed under the `CeCILL-C` license, for which can find a copy here:
<https://gitlab.inria.fr/scotch/scotch/-/raw/v6.1.0/doc/CeCILL-C_V1-en.txt>

[Scotch]: https://gitlab.inria.fr/scotch/scotch/
