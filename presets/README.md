# Presets

Presets are [toml](https://toml.io/) files used to configure bluerepl for a specific peripheral.

TOML is easy to learn, write and read which makes it great for configuration files like this.


To use a preset with bluerepl just specify the path to the preset file when starting the program.

```shell
$ bluerepl battery.toml
```

You can type **preset** in bluerepl to display all the configuration loaded.

```
$ bluerepl battery.toml
>> preset
...
```

# Create a preset

First take a look at existing [presets](presets/).

Presets files can contain 4 main sections:

1. [Device](#device)
2. [Services](#services)
3. [Commands](#commands)
4. [Functions](#functions)


## Device

In this section are defined informations about the peripheral.

- **name** is the name of the peripheral
- **address** is the mac address (or the OSX UUID) of the peripheral

Both are optional but at least one must be provided to use the **-a --autoconnect** feature.

Example

```toml
[device]
name = "my_device"
address = "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
```

## Services

Services section contains the definition of the GATT (Generic ATTribute Profile) of the BLE peripheral you want to use.

To declare a service and it's characteristics the TOML [inline table](https://toml.io/en/v1.0.0#inline-table) syntax is used.

```toml
# create a service called 'my_service' with an uuid
# uuid is the only field and is mandatory for services
[services.my_service]
uuid = "8e72bbe5-f777-5284-7849-b4a0b2ac70d2"
```

Then to create service characteristics add **.characteristic.characteristic_name** in the table name.

```toml
# create a characteristic called 'write' associated to the service 'my_service' with an uuid
# uuid is the only field and is mandatory for characteristics
[services.my_service.characteristics.write]
uuid = "0000beb6-0000-1000-8000-00805f9b34fb"
```

## Commands

Commands is what makes presets so useful. Quick example of their usage before explaining the syntax and everything.

To write a service without a command:
```
>> write color_service rgb_characteristic "0xff 0x00 0x00"
```
with a command:
```
>> preset command set_red
```

Which is much clearer and user friendly.

The command used in this example would look like this

```toml
[commands.set_red]
command_type = "write"
service = "color_service"
characteristic = "rgb_characteristic"
payload = "0xff 0x00 0x00"
```

*command_type* can be any of:
 - **write** 
 - **read**
 - **notify**
 - **indicate**
 - **unsubscribe**

The fields **command_type**, **service** and characteristic** are mandatory for all commands.

Commands of types **read**, **notify** and **service** can also have a format field which corresponds to the **-f** flag in the repl.
By default the format is set to *hex* it can be set to any of **hex**, **text**, **binary**, **decimal** and **hexdump**

Example:

```toml
[commands.read_manufacturer_name]
command_type = "read"
service = "dis"
characteristic = "manufacturer_name"
format = "text"
```

## Functions

Functions are a group of commands called one after the other with a specified delay between each.

If the commands **set_red**, **clear** and **set_blue** are defined we can write a function to make the led blink like this.

```toml
[functions.blink_rb]
commands_delay_ms = [1000, 500, 1000, 0]
commands = ["set_red", "clear", "set_blue", "clear"]

# this will be executed in this order:
# set_red
# wait 1000ms
# clear
# wait 500ms
# set_blue
# wait 1000ms
# clear
# wait 0ms
```

Call this function with

```
>> preset function blink_rb
```
