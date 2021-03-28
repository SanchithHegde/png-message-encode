# PNG Message Encode

My implementation of
[picklenerd/pngme_book](https://github.com/picklenerd/pngme_book)
([rendered book](https://picklenerd.github.io/pngme_book/introduction.html)),
for educational purposes.

## Table of Contents <!-- omit in toc -->

- [PNG Message Encode](#png-message-encode)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Encode a message into a PNG file](#encode-a-message-into-a-png-file)
    - [Decode a message stored in a PNG file](#decode-a-message-stored-in-a-png-file)
    - [Remove a message from a PNG file](#remove-a-message-from-a-png-file)
    - [Print a list of PNG chunks that can be searched for messages](#print-a-list-of-png-chunks-that-can-be-searched-for-messages)
  - [Running tests](#running-tests)
  - [License](#license)

## Installation

To build this project from source, you need Rust 1.40 or higher. Refer to
[the docs](https://www.rust-lang.org/tools/install) for more information on
installing Rust.

You can then use cargo to build everything:

```shell
git clone https://github.com/SanchithHegde/png-message-encode
cd png-message-encode
cargo install --path .
```

## Usage

### Encode a message into a PNG file

```text
$ pngme encode
pngme-encode
Encode a message in a PNG file

USAGE:
    pngme encode [FLAGS] <in-file> <chunk-type> <message> [out-file]

ARGS:
    <in-file>       Path to the PNG file to encode the message in
    <chunk-type>    A 4-character long ASCII alphabetic string
    <message>       Message to encode
    <out-file>      Path to the PNG file to save the encoded image as. Optional. If this is not
                    specified, the input PNG file is updated in place

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Prints verbose information
    -V, --version    Prints version information
```

So, you'd run:

```text
pngme encode /path/to/image.png teXt "This is a secret message!"
```

Alternatively, if you want the encoded image to be stored as a separate file,
you'd run:

```text
pngme encode /path/to/image.png teXt "This is a secret message!" /path/to/image_out.png
```

### Decode a message stored in a PNG file

```text
$ pngme decode
pngme-decode
Decode a message in a PNG file

USAGE:
    pngme decode [FLAGS] <in-file> <chunk-type>

ARGS:
    <in-file>       Path to the PNG file to decode the message from
    <chunk-type>    A 4-character long ASCII alphabetic string

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Prints verbose information
    -V, --version    Prints version information
```

So, you'd run:

```text
$ pngme decode /path/to/image.png teXt
This is a secret message!
```

### Remove a message from a PNG file

```text
$ pngme remove
pngme-remove
Remove a message from a PNG file

USAGE:
    pngme remove [FLAGS] <in-file> <chunk-type>

ARGS:
    <in-file>       Path to the PNG file to remove the message from
    <chunk-type>    A 4-character long ASCII alphabetic string

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Prints verbose information
    -V, --version    Prints version information
```

So, you'd run:

```text
pngme remove /path/to/image.png teXt
```

### Print a list of PNG chunks that can be searched for messages

```text
$ pngme print
pngme-print
Print a list of PNG chunks that can be searched for messages

USAGE:
    pngme print [FLAGS] <in-file>

ARGS:
    <in-file>    Path to the PNG file to list all chunks

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Prints verbose information
    -V, --version    Prints version information
```

So, you'd run (provided there's a `teXt` chunk added to the image):

```text
$ pngme print /path/to/image.png
PNG chunks found in file '/path/to/image.png':

teXt
```

## Running tests

```shell
cargo test
```

## License

Dual licensed under Apache 2.0 or MIT at your option.

See the [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) files for license details.
