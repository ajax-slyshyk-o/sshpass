# sshpass-rs

A Rust reimplementation of [sshpass](https://sourceforge.net/projects/sshpass/) — a tool for non-interactive SSH password authentication. It injects a password into SSH without requiring manual input, making it useful for scripts and automation.

## How It Works

`sshpass-rs` uses SSH's `SSH_ASKPASS` mechanism. It sets itself as the askpass helper, then when SSH asks for a password it calls back into the same binary, which prints the password and exits. This avoids PTY manipulation and works on all platforms that support `SSH_ASKPASS_REQUIRE=force`.

## Installation

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# Binary: ./target/release/sshpass-rs
```

## Usage

### Pass password via argument

```bash
sshpass -p <password> ssh user@host
```

### Pass password via environment variable

```bash
export SSHPASS=mysecretpassword
sshpass -e ssh user@host
```

### With additional SSH flags

```bash
sshpass -p mypassword ssh -p 2222 user@host command
sshpass -p mypassword scp file.txt user@host:/tmp/
```

## Options

| Flag | Description |
|------|-------------|
| `-p <password>` | Use the given password |
| `-e` | Read the password from the `SSHPASS` environment variable |

## Security Notes

- Passing passwords on the command line (`-p`) exposes them in the process list. Prefer `-e` with a secured environment variable in production scripts.
- This tool is intended for automation scenarios where interactive input is not possible. Prefer SSH key-based authentication whenever feasible.

## Requirements

- Rust 2024 edition
- An SSH client in `PATH` (e.g. `ssh`, `scp`, `sftp`)
- No external crate dependencies — stdlib only

## License

MIT
