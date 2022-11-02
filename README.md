# bluerepl

bluerepl is a command line application to interact with Bluetooth Low Energy (BLE) peripherals.

# Build and install

```shell
$ git clone https://github.com/Yohannfra/bluerepl

$ cargo build

$ cargo install --path .
```

# Basic usage

```bash
# print help about all commands
>> help

# print help about a specific command
>> help scan

# scan peripherals around
>> scan

# connect to a peripherals with it's name (found in scan)
>> connect -n PERIPH_NAME
```
