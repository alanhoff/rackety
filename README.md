# Rackety

Rackety is a blazing fast, multi thread and simple in-memory key-value storage.

### Usage

Just [download the binary]() for your platform or build it yourself and run it.

```text
USAGE:
    rackety [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <Interface>    The interface where the server will bind to. Defaults to 127.0.0.1
    -p, --port <Port>              The port where the server will listen to. Defaults to 6771
    -t, --threads <Threads>        The number of threads to use. Defaults to the amount of cores in the machine
```

### Protocol

Rackety uses a text protocol that's POSIX compatible. You can read more about
the commands that you can perform [here](docs/protocol.md).

### License

```text
ISC License

Copyright (c) 2016, Alan Hoffmeister <alanhoffmeister@gmail.com>

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```
