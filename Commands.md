# bluerepl command documentation

## clear

```
Clear the terminal 

USAGE:
    clear

OPTIONS:
    -h, --help    Print help information
```

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

## disconnect

```
Disconnect from BLE peripheral

USAGE:
    disconnect

OPTIONS:
    -h, --help    Print help information
```

## help

```
Print this message or the help of the given subcommand(s)

USAGE:
    help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display
```

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

## quit

```
Quit the REPL

USAGE:
    quit

OPTIONS:
    -h, --help    Print help information
```

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
