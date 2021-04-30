# boyo - a work-in-progress emulator

A cycle-accurate, efficient, and memory safe emulator for the Gameboy and (eventually) the Gameboy Advance.

## Usage

You can run the emulator using the following command:

```
boyo path/to/game.rom
```

Note: boyo is **very much** work in progress. Running the emulator this way will not work very well.

## Debugging

You can start the emulator in debug mode as well:

```
boyo path/to/game.rom --debug
```

This will put your terminal into debugger mode, from which you can execute various commands to control execution of the program.

Available commands:
* `break-add <address>` - Adds a new breakpoint at the given (hex) address.
* `break-list` - Shows all the currently active breakpoints.
* `break-remove <address>` - Removes an existing breakpoint at the given (hex) address, if it exists.
* `continue` - Begins execution until a breakpoint is hit.
* `exit` - Exits the program.
* `help` - How you got here.
* `next` - Displays the next instruction to be executed.
* `registers` - Displays the contents of all cpu registers.
* `step` - Executes a single instruction.\n