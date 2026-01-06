# chip_eight

A Rust emulator of Chip Eight (Command-Line only)

# Inspiried by

https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

# Database with roms

https://chip-8.github.io/database/

# Display

Includes two display versions, one ASCII, and one utf-8 which uses 'x' as the
representative character for a pixel that is turned on.

The ASCII version (requires a code modification to use), calling
chip.terminal_display instead of chip.display. But does not offer
interactability.

# Instruction set

0NNN: Execute machine language routine

00E0: Clear screen

1NNN: Jump

00EE: Subroutines

2NNN: Subroutines

3XNN: Skip conditionally

4XNN: Skip conditionally

5XY0: Skip conditionally

9XY0: Skip conditionally

6XNN: Set

7XNN: Add

8XY0: Set

8XY1: Binary OR

8XY2: Binary AND

8XY3: Logical XOR

8XY4: Add

8XY5: Subtract

8XY7: Subtract

8XY6: Shift

8XYE: Shift

ANNN: Set index

CXNN: Random

DXYN: Display

EX9E: Skip if key

EXA1: Skip if key

FX07: Timers

FX15: Timers

FX18: Timers

FX1E: Add to index

FX0A: Get key

FX29: Font character

FX33: Binary-coded decimal conversion

FX55: Store and load memory
FX65: Store and load memory
