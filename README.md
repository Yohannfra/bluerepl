# bluerepl

bluerepl is a command line application to interact with Bluetooth Low Energy (BLE) peripherals.

It works on **Windows**, **Mac OS** and **Linux**.

## Installation

### Using crate.io

```bash
$ cargo install bluerepl
```

### From source

```bash
$ git clone https://github.com/Yohannfra/bluerepl

$ cargo build

$ cargo install --path .
```

## Quick Start

Launch the application with the **bleurepl** command, once started it will display basic informations.

```bash
$ bluerepl
bleurepl Version: 0.1.2
Using BLE adapter: "CoreBluetooth"
>> ...
```

You can type **help** to see all available commands and their usages or a help text about a specific command.

```
>> help
COMMANDS:
    clear          Clear the terminal
    connect        Connect to a BLE peripheral
    disconnect     Disconnect from BLE peripheral
    help           Print this message or the help of the given subcommand(s)
    indicate       Subscribe to a characteristic indications and print it's value when it gets updated
    info           Print informations about a specified topic
    notify         Subscribe to a characteristic notifications and print it's value when it gets updated
    preset         Print preset informations or run preset commands/functions
    quit           Quit the REPL
    read           Read the value of a characteristic
    scan           Search for BLE devices around
    sleep          Wait and do nothing for a specified amount of time
    unsubscribe    Unsubscribe from the notifications or indications of a characteristic
    write          Write a value to a characteristic

>> help scan
Search for BLE devices around

USAGE:
    scan [OPTIONS] [timeout]

ARGS:
    <timeout>    Time to scan in seconds [default: 5]

OPTIONS:
    -a, --all     Show unnamed peripheral
    -h, --help    Print help information
    -l, --list    Show last scan list (doesn't run a new scan)
...
```

A *typical* workflow would look like this:

```bash
>> scan # search for peripherals 
Scanning for 5 seconds...
....

>> connect 12 # connect to peripheral with id 12 in scan list
Connected

>> info gatt # print all gatt, services and characteristics of the connected peripheral
...

>> read 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb # read a characteristic value
[0xff, 0x32, 0x31 ...]

>> write 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb "0x12 0x44" # write a characteristic value

>> disconnect # disconnect from device

>> quit # or CTRL+D
```

## Commands

### A complete list of all the commands with examples can be found in [Commands.md](./Commands.md)

## License

see [LICENSE](./LICENSE)
