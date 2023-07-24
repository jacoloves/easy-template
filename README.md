# Easy-Template
Easy-Template allows you to register templates and select and recall registered templates.

## Purpose
- for fun
- for lean Rust
- It is convenient to have it

## Features
- [x] Template registration function
- [x] Template call function (no extension specified)
- [x] Template call function (with extended specification)

## Usage
### Register template
```bash
$ easy-template -r <filename>
```

### Call template
```bash
$ easy-template -c
```
or
```bash
$ easy-template -c <exetension_name>
```

## Installation
```bash
$ cargo install --git
```

## License
Distributed under the MIT License. See `LICENSE`.