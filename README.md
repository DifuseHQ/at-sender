# AT Sender

The `at_sender` program is a simple command-line tool written in Rust, designed to send AT commands to 4G/5G modems on embedded devices. It is intended to be used as a diagnostic tool for testing the modem's AT command interface.

## Requirements

- Rust 1.41 or later
- Access to a serial device
- Linux operating system (for the provided build commands)

## Installation

Clone the repository and navigate into the project directory:

```bash
git clone https://git.difuse.io/Difuse/at-sender.git
cd at-sender
```

For easy native builds, use the following command:

```bash
cargo build --release
```

Compiling for other targets is also possible, but requires additional setup (You are on your own for this one), here's a great docker container that you can use to build for `aarch64-musl`:

```bash
docker run --rm -v $(pwd):/home/rust/src messense/rust-musl-cross:aarch64-musl cargo build --release
```

## Usage

```
./at_sender --port /dev/ttyUSB2 --baud 115200 --data 8 --stop 1 --cmd "AT" --buf 32
```

### Flags

* `--port`: The serial port to connect to (default "/dev/ttyUSB2")
* `--baud`: The baud rate (default 115200)
* `--data`: The number of data bits (default 8)
* `--stop`: The number of stop bits (default 1)
* `--cmd`: The command to send to the serial device (default "AT")
* `--buf`: The buffer size for reading from the serial device (default 4)

The flags can be specified in any order. If a flag is omitted, the default value will be used.

## License

GPL-3.0