# png2tileset

![Original Tile Map](docs/Sprite-0001.png)
 \- Original Tile Map

![8x8 Tileset created from the Tile Map](docs/Sprite-0001-tileset-8x8.png)
 \- 8x8 Tileset created from the Tile Map

![4x4 Tileset created from the Tile Map](docs/Sprite-0001-tileset-4x4.png)
 \- 4x4 Tileset created from the Tile Map

## Description

This small utility allows you to generate tilesets out of PNG images. It ignores duplicated tiles and creates a smallest possible tileset of given size. It's made for my personal purposes to create tilesets for GBA games but probably can used for something else.

## Usage

You want to build this with `cargo build --release` before use. After this the release version of a binary can be found in `target/release` directory.

```bash
Converts png images (tilemaps) into png tilesets

Usage: png2tileset [OPTIONS] <FILE>

Arguments:
  <FILE>  File path

Options:
  -o, --output <OUTPUT>  Output file path
  -s, --size <SIZE>      Tile size (in pixels) [default: 8]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Development

Should I even write it?

```bash
cargo run

cargo build

cargo build --release
```

## License

[WTFPL](https://en.wikipedia.org/wiki/WTFPL)
