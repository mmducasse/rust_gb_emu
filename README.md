# Rust Game Boy Emulator

An emulator for the original Nintendo Game Boy (1989) written in Rust.

![preview](https://github.com/mmducasse/rust_gb_2/blob/master/assets/gifs/emu_gif.gif)

### Features
- Displays live VRAM tile map and tile data.
- Save and load cartridge RAM contents.
- Supported cartridge types: ROM-only, MBC1, and MBC3.

### Controls
| Game Boy | Keyboard |
| ----------- | ----------- |
| D-Pad | Arrows |
| B | Z |
| A | X |
| Start | Enter |
| Select | Right Shift |
| Save | Backspace |
| Toggle speedup | Space |
| Toggle tile map view | T |

### Useful Resources
- gbdev.io Pan Docs: https://gbdev.io/pandocs/About.html
- CPU Opcodes map: https://meganesu.github.io/generate-gb-opcodes/
- Cart Memory Bank Controllers: https://gbdev.gg8.se/wiki/articles/Memory_Bank_Controllers
- Blargg's Test ROMS: https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs

### Not implemented
- Color Games
- Audio
