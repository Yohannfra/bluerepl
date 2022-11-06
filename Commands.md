# bluerepl command documentation

## Summary

- [clear](#clear)
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

OPTIONS:
    -h, --help    Print help information
```

Example:
```bash
# clear the screen
>> clear
```

---

## connect

```
USAGE:
    connect --name <name> --mac <mac> --id <id> <identifier>

ARGS:
    <identifier>    Parse identifier and use it to connect with name, mac or id

OPTIONS:
    -h, --help           Print help information
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

OPTIONS:
    -h, --help    Print help information
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
    -h, --help    Print help information
```

---

## info

```
Print informations about a specified topic

USAGE:
    info <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    adapter    Print informations about BLE adapter in use
    gatt       Print informations about the gatt of the connected peripheral
    help       Print this message or the help of the given subcommand(s)
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
    -h, --help    Print help information
```

---

## preset

```
Print preset informations or run preset commands/functions

USAGE:
    preset [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    command     Run preset command
    function    Run preset function
    help        Print this message or the help of the given subcommand(s)
```

---

## quit

```
Quit the REPL

USAGE:
    quit

OPTIONS:
    -h, --help    Print help information
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
    -h, --help    Print help information
```

Example:
```bash
# read the value of a characteristic with a 16 bits uuid
>> read 180f 2a19

# read the value of a characteristic with a long uuid
>> read 0000180a-0000-1000-8000-00805f9b34fb 00002a24-0000-1000-8000-00805f9b34fb
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
    -h, --help    Print help information
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

OPTIONS:
    -h, --help    Print help information
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
    -h, --help      Print help information
    -n, --noresp    Write no response (default write is write with response)
```
