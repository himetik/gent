### Gent is a simple CLI password generator

### Installation

>The user must have Rust installed. If Rust is not installed, it can be installed using Rustup by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

#### Clone the repository:

```sh
git clone https://github.com/himetik/gent.git
```

#### Build and install the program:

Ensure that `~/.cargo/bin` is added to your `PATH` environment variable.

```sh
cargo install --path .
```

### Usage

Generate a password of 16 characters:

```sh
gen 16
```

Get help with the command:

```sh
gent --help
```
