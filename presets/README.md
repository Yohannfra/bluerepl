# Presets

Presets are [toml](https://toml.io/) files used to configure bluerepl for a specific peripheral.


To use a preset with bluerepl just specify the path to the preset file when starting the program.

```shell
$ bluerepl battery.toml
```

You can type **preset** in bluerepl to display all the configuration loaded.

```
$ bluerepl battery.toml
>> preset
+--------------------+--------------------------------------+
| File name:         | presets/battery.toml                 |
+===========================================================+
| Device name        | my_device                            |
| Device address     | XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX |
|--------------------+--------------------------------------|
| Service            |                                      |
|--------------------+--------------------------------------|
| Name               | battery                              |
| UUID               | 180f                                 |
|                    |                                      |
| Characteristic:    |                                      |
|  - Name:           | level                                |
|  - UUID            | 2a19                                 |
|--------------------+--------------------------------------|
| Commands           |                                      |
|--------------------+--------------------------------------|
| Name               | sub_battery                          |
| Type               | notify                               |
| Service            | battery                              |
| Characteristic     | level                                |
| Payload            |                                      |
|--------------------+--------------------------------------|
| Name               | read_battery                         |
| Type               | read                                 |
| Service            | battery                              |
| Characteristic     | level                                |
| Payload            |                                      |
+--------------------+--------------------------------------+
```
