Proof of concept to see if it is possible to add palette parser (written in rust) to an amstrad CPC emulator (writtent in C).

Project is fully viable. However the parse binary is quite huge in comparison to emulots (half the size).

- <cpcemu_parser> contains the parser and interpreter  written in `rust`. It exports a function to call on the `c` side `const int8_t *cpcemu_execute_line(const int8_t *line);`. Once called, it parse it and launch the appropriate `c` functions implemented within the emulator. The grammar is written in `pest`: <./cpcemu_parser/src/cpcemu.pest>.
- <fake_emulator> contains a simple hello world that calls `cpcemu_execute_line` and provides a set of functions to be called from the rust side.
- <cpcemu_public.h> : the functions to be implemented by the emulator and called by the interpreter.

