# switsshfs

Easily mount and unmount a directory using sshfs.

## Installation

Use the Rust toolchain to build from sources:

```bash
cargo build --release
```

The executable file is written to `target/release/switsshfs` and is ready to be moved in your PATH.

## Usage

### Prerequisites

The following commands must be available in your PATH:
- `sshfs`.
- `fusermount`.

### Set up a mountpoint

You can configure a directory to be a mountpoint by putting a `switsshfs.toml` file in it.
The configuration file looks like this:

```toml
remote = "me@my.server.net:project/src"
```

1. Create a directory that will serve as a mountpoint.
2. In that directory, write a switsshfs config file named `switsshfs.toml`.
3. Set up the `remote` configuration.

Example:

```bash
mkdir remote-src
echo 'remote = "me@my.server.net:project/src"' > remote-src/switsshfs.toml
```

### Mount and unmount a mountpoint

A mountpoint can be switched on and off by giving it as argument to the `switsshfs` command:

```bash
switsshfs remote-src
```

## Credits

The idea comes from Nicolas Albert.
I stumbled upon their [fusauto](https://doc.ubuntu-fr.org/fusauto) script while researching things about sshfs.
I just thought it would be fun to write my own version to practice Rust.
