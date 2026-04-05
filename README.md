# actualizador

Maintenance and update tool for Debian systems, written in Rust.

## Description

`actualizador` is a command-line utility that automates common maintenance tasks on Debian-based systems, including:

- System package updates using APT
- Removal of unused packages (`apt autoremove`)
- Flatpak update and cleanup
- Snap package updates
- Rust toolchain updates via `rustup`

## Usage

```sh
cargo run --release [--autoremove]
```

- `--autoremove` or `-a`: Runs `apt autoremove` after updating system packages.

### Example

```sh
cargo run --release -- --autoremove
```

## Requirements

- Operating system: Debian or derivative
- Rust (to compile)
- Dependencies:
  - [clap](https://crates.io/crates/clap)
  - [colored](https://crates.io/crates/colored)

## Installation

1. Clone this repository:
   ```sh
   git clone <repo-url>
   cd actualizador
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```
3. Run the generated binary in `target/release/actualizador`.

## Notes
- The program automatically detects if you have `flatpak`, `snap`, or `rustup` installed and runs the corresponding actions.
- It is recommended to run as superuser so system commands work correctly.

## License

MIT
