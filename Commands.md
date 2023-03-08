# bluerepl command documentation

## Summary

- [clear](#clear)
- [sleep](#sleep)
- [connect](#connect)
- [disconnect](#disconnect)
- [help](#help)
- [indicate](#indicate)
- [info](#info)
- [notify](#notify)
- [preset](#preset)
- [quit](#quit)
- [read](#read)
- [scan](#scan)
- [unsubscribe](#unsubscribe)
- [write](#write)

---

## clear

```
Clear the terminal

USAGE:
    clear
```

Example:
```bash
# clear the screen
>> clear
```

---

## sleep

```
Wait and do nothing for a specified amount of time

USAGE:
    sleep <time_ms>

ARGS:
    <time_ms>    Time to sleep in milliseconds

OPTIONS:
    -h, --help    Print help information
```

Example:
```bash
# sleep for 500ms
>> sleep 500
```

---

## connect

```
USAGE:
    connect --name <name> --mac <mac> --id <id> <identifier>

ARGS:
    <identifier>    Parse identifier and use it to connect with name, mac or id

OPTIONS:
    -i, --id <id>        Connection using the id of the peripheral in the scan list
    -m, --mac <mac>      Connection using the mac address of the peripheral
    -n, --name <name>    Connection using the name of the peripheral
```

Examples:
```bash
# connect to peripheral with id 3 in the scan list
>> connect -i 3
# also works without -i
>> connect 3

# connect to peripheral named hrs_sensor
>> connect -n hrs_sensor
# also works without -n
>> connect hrs_sensor

# connect to peripheral with mac address 11:22:33:44:55:66
>> connect -m 11:22:33:44:55:66
# also works without -m
>> connect 11:22:33:44:55:66
```

---

## disconnect

```
Disconnect from BLE peripheral

USAGE:
    disconnect
```

Example:
```bash
# when connected to a peripheral
>> disconnect
```

---

## help

```
Print this message or the help of the given subcommand(s)

USAGE:
    help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display
```

Examples:
```bash
# global help message
>> help

# help about a specific command
>> help read
```
---

## indicate

```
Subscribe to a characteristic indications and print it's value when it gets updated

USAGE:
    indicate <service> <characteristic>

ARGS:
    <service>           The service that contains the characteristic to subscribe to
    <characteristic>    The characteristic to subscribe to

OPTIONS:
    -f, --format <format>    Format to print the read value [default: hex] [possible values: bin,
                             hex, dec, text, hexdump]
```

Example:
```bash
# subscribe to a service indications
>> indicate 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb

# subscribe to a service indications and print indications as text
>> indicate 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb -f text
```

---

## info

```
Print informations about a specified topic

USAGE:
    info <SUBCOMMAND>

SUBCOMMANDS:
    adapter    Print informations about BLE adapter in use
    gatt       Print informations about the gatt of the connected peripheral
```

Examples:
```bash
# print informations about the ble adapter in use
>> info adapter

# print informations about gatt of the connected peripheral, this will print all it's services and characteristics
>> info gatt
```

---

## notify

```
Subscribe to a characteristic notifications and print it's value when it gets updated

USAGE:
    notify <service> <characteristic>

ARGS:
    <service>           The service that contains the characteristic to subscribe to
    <characteristic>    The characteristic to subscribe to

OPTIONS:
    -f, --format <format>    Format to print the read value [default: hex] [possible values: bin,
                             hex, dec, text, hexdump]
```

Example:
```bash
# subscribe to a service notifications
>> notify 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb

# subscribe to a service notifications and print notifications as binary
>> notify 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb -f bin
```

---

## preset

```
Print preset informations or run preset commands/functions

USAGE:
    preset [SUBCOMMAND]

SUBCOMMANDS:
    command     Run preset command
    function    Run preset function
```

Examples:
```bash
# display preset content
>> preset

# run a command called 'blink_red' defined in preset
>> preset command blink_red

# run a function called 'blink_all' defined in preset
>> preset function blink_all
```

---

## quit

```
Quit the REPL

USAGE:
    quit
```

Example:
```bash
# close the repl and exit the program
>> quit
```

---

## read

```
Read the value of a characteristic

USAGE:
    read <service> <characteristic>

ARGS:
    <service>           The service that contains the characteristic to write
    <characteristic>    The characteristic to write

OPTIONS:
    -f, --format <format>    Format to print the read value [default: hex] [possible values: bin,
                             hex, dec, text, hexdump]
```

Example:
```bash
# read the value of a characteristic
>> read 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb

# read the value of a characteristic and print the value as decimal
>> read 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb -f dec

# read the value of a characteristic and print the value as text
>> read 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb -f text
```

---

## scan

```
Search for BLE devices around

USAGE:
    scan [OPTIONS] [timeout]

ARGS:
    <timeout>    Time to scan in seconds [default: 5]

OPTIONS:
    -a, --all     Show unnamed peripheral
    -l, --list    Show last scan list (doesn't run a new scan)
```

Examples:
```bash
# scan for 5 seconds
>> scan

# scan for 2 seconds
>> scan 2

# scan for 4 seconds and display unnamed peripheral
>> scan 4 -a

# print previous scan list
>> scan -l
```

---

## unsubscribe

```
Unsubscribe from the notifications or indications of a characteristic

USAGE:
    unsubscribe <service> <characteristic>

ARGS:
    <service>           The service that contains the characteristic to unsubscribe from
    <characteristic>    The characteristic to unsubscribe from
```

Example:
```bash
# unsubscribe from a service notifications or indications
>> unsubscribe 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb
```

---

## write

```
Write a value to a characteristic

USAGE:
    write [OPTIONS] <service> <characteristic> <payload>

ARGS:
    <service>           The service that contains the characteristic to write
    <characteristic>    The characteristic to write
    <payload>           The payload to write

OPTIONS:
    -r, --resp    Write with response (default write is write without response)
```

The payload of the **write** command is given to a ['parser'](https://github.com/Yohannfra/str_to_bytes) that will convert all the values written as string to bytes.

It detects 'identifier' and convert the values based on them, the supported identifier are:
- Hexadecimal with **0x** and **0X**. ```0xff 0X44 0x23 ...```
- Binary with **0b** and **0B**. ```0b10 0B1001 0b1010111011001```
- Decimal with written decimal number. ```12 24 55624 ...```
- Ascii string with a special **ASCII()** syntax. ```ASCII(Hello) ASCII(Hi mom)```

All values larger than a single byte (0xff / 255 / 0b11111111) will be correctly splitted in as many bytes as needed:

```
0b1111111101 => 0b11111111 0b01
0xffabcd => 0xff 0xab 0xcd
256 => 255 1
...
```

**And all these syntaxes can be used together in the same command.**


Examples:
```bash
# let's assume for these examples that a is service uuid and b is characteristic uuid

# simply write 0xff to the characteristic
>> write a b 0xff
>> write a b 255
>> write a b 0b11111111

# write hello world (notice that the quotes are needed for the space)
>> write a b "ASCII(hello world)"
# or just hello
>> write a b ASCII(hello)

# write multiple bytes
>> write a b "0xff00ff"
# is the same as
>> write a b "0xff 0x00 0xff"

# also works for binary and decimal
>> write a b "0b11 0b11100 0b110011001010111001011010"
>> write a b "12 32 429 21313"

# mixing everyting together
>> write "0b11 1242 0 0x45 0xff422 ASCII(HI)"

...
```
