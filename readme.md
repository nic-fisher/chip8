# CHIP-8 Rust

A CHIP-8 interpreter written in Rust.

### Usage

`cargo run <rom_file> <cycles_per_frame>`

`cycles_per_frame` defaults to 14.

<`Esc`> to exit.

### Keyboard mapping

Emulator     Chip8
+-+-+-+-+    +-+-+-+-+
|1|2|3|4|    |1|2|3|C|
|Q|W|E|R|    |4|5|6|D|
|A|S|D|F|    |7|8|9|E|
|Z|X|C|V|    |A|0|B|F|
+-+-+-+-+    +-+-+-+-+

### Notes

The CPU cycle timing is not accurate and the sound timer is not setup.

### References

https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
https://austinmorlan.com/posts/chip8_emulator/
http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.1