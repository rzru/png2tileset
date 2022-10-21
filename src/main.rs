use std::{error::Error, ffi::OsStr, io, path::Path, process};

use clap::Parser;
use image::{imageops, open, GenericImageView, ImageBuffer};

/// Creates sick tilesets out of png images
#[derive(Parser, Debug)]
#[command(name = "png2tileset")]
#[command(author = "rzru <rzzzzru@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Converts png images (tilemaps) into png tilesets", long_about = None)]
struct Args {
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,

    /// Tile size (in pixels)
    #[arg(short, long, default_value_t = 8)]
    size: u32,

    /// File path
    file: String,
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&args.file);
    let image = open(&args.file)?;
    let tile_size = args.size;

    let make_error = |text| {
        Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("ERROR: {}", text),
        ))
    };

    if image.width() % tile_size != 0 {
        return Err(make_error(
            "Image width must be a multiple of the tile size",
        ));
    }

    if image.height() % tile_size != 0 {
        return Err(make_error(
            "Image height must be a multiple of the tile size",
        ));
    }

    let x_tiles_count = image.width() / tile_size;
    let y_tiles_count = image.height() / tile_size;

    let mut res = vec![];

    for y_tile in 0..y_tiles_count {
        for x_tile in 0..x_tiles_count {
            let mut inner = ImageBuffer::new(tile_size, tile_size);

            for x in 0..tile_size {
                for y in 0..tile_size {
                    inner.put_pixel(
                        x,
                        y,
                        image.get_pixel(x + (x_tile * tile_size), y + (y_tile * tile_size)),
                    );
                }
            }

            if !res.contains(&inner) {
                res.push(inner);
            }
        }
    }
    let pixels = res.len() as u32 * tile_size;

    let (mut width, mut height) = (pixels, tile_size);

    for height_pretender in 0..pixels {
        for width_pretender in 0..pixels {
            if height_pretender * width_pretender >= pixels * tile_size
                && height_pretender % tile_size == 0
                && width_pretender % tile_size == 0
                && width_pretender + height_pretender < width + height
            {
                width = width_pretender;
                height = height_pretender;
            }
        }
    }

    let mut result_image = ImageBuffer::new(width, height);
    let mut y = 0;
    let mut x = 0;
    for tile in res.iter() {
        if x == width {
            y += tile_size;
            x = 0;
        }
        imageops::overlay(&mut result_image, tile, x as i64, y as i64);
        x += tile_size;
    }

    let default_file_path = format!(
        "{}-tileset-{}x{}.png",
        path.file_stem()
            .unwrap_or(OsStr::new("my"))
            .to_str()
            .unwrap_or("my"),
        tile_size,
        tile_size
    );
    let output_path = args.output.as_ref().unwrap_or(&default_file_path);
    result_image.save(output_path)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let result = run(&args);

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(1);
    }

    process::exit(0)
}
