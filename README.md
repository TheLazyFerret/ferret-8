# ferret-8
Small and simple, emulator for the original CHIP-8 written in rust.

## Installation
You can simply install it with cargo:
```bash
git clone https://github.com/TheLazyFerret/ferret-8.git
cd ferret-8
cargo install --path .
```

## Usage
```bash
Usage: ferret-8 [OPTIONS] --program <PROGRAM>

Options:
  -p, --program <PROGRAM>
          Program path
  -c, --cycles <CYCLES>
          Cycles (instructions) per second the program will execute [default: 700]
  -u, --upscale-factor <UPSCALE_FACTOR>
          Upscale factor from the original 64x32 pixel size [default: 20]
  -m, --modern-compatibility
          Modern behaviour in some instructions
  -h, --help
          Print help
  -V, --version
          Print version
```

## Controls
Translate the original COSMAC-VIP keypad:
```
KEYPAD     KEYBOARD
1 2 3 C    1 2 3 4
4 5 6 D    q w e r
7 8 9 E    a s d f
A 0 B F    z x c v
```

## To-do
- [ ] Sounds.
- [ ] Add more keyboard layouts support.
- [ ] Toggle for a few newer instructions.
- [x] Better instruction compatibility.

## License
This project is under the [MIT](LICENSE) license. See the dependencies in [Cargo.toml](Cargo.toml) for each individual license.