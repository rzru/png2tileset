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

#[cfg(test)]
mod tests {
    use image::open;

    use crate::{run, Args};
    use std::env;
    use std::fs::remove_file;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn setup() -> () {
        INIT.call_once(|| {
            assert!(env::set_current_dir("./tests").is_ok());
        });
    }

    #[test]
    fn returns_error_if_width_is_not_multiple_of_tile_size() {
        setup();

        let args = Args {
            output: None,
            size: 8,
            file: String::from("1.png"),
        };

        assert_eq!(run(&args).is_err(), true);
    }

    #[test]
    fn returns_error_if_height_is_not_multiple_of_tile_size() {
        setup();

        let args = Args {
            output: None,
            size: 8,
            file: String::from("2.png"),
        };

        assert_eq!(run(&args).is_err(), true);
    }

    #[test]
    fn returns_ok_and_creates_correct_tileset_into_default_output_size_16() {
        setup();

        let args = Args {
            output: None,
            size: 16,
            file: String::from("3.png"),
        };

        let output_name = "3-tileset-16x16.png";

        assert!(run(&args).is_ok());
        assert_eq!(
            open(output_name).unwrap().as_bytes(),
            open("3-assertion-output-16x16.png").unwrap().as_bytes()
        );

        assert!(remove_file(output_name).is_ok());
    }

    #[test]
    fn returns_ok_and_creates_correct_tileset_into_default_output_size_8() {
        setup();

        let args = Args {
            output: None,
            size: 8,
            file: String::from("3.png"),
        };

        let output_name = "3-tileset-8x8.png";

        assert!(run(&args).is_ok());
        assert_eq!(
            open(output_name).unwrap().as_bytes(),
            open("3-assertion-output-8x8.png").unwrap().as_bytes()
        );

        assert!(remove_file(output_name).is_ok());
    }

    #[test]
    fn returns_ok_and_creates_correct_tileset_into_default_output_size_4() {
        setup();

        let args = Args {
            output: None,
            size: 4,
            file: String::from("3.png"),
        };

        let output_name = "3-tileset-4x4.png";

        assert!(run(&args).is_ok());
        assert_eq!(
            open(output_name).unwrap().as_bytes(),
            open("3-assertion-output-4x4.png").unwrap().as_bytes()
        );

        assert!(remove_file(output_name).is_ok());
    }

    #[test]
    fn returns_ok_and_creates_correct_tileset_into_custom_output() {
        setup();

        let output_name = "custom-tileset-name.png";
        let args = Args {
            output: Some(String::from(output_name)),
            size: 4,
            file: String::from("3.png"),
        };

        assert!(run(&args).is_ok());
        assert_eq!(
            open(output_name).unwrap().as_bytes(),
            open("3-assertion-output-4x4.png").unwrap().as_bytes()
        );

        assert!(remove_file(output_name).is_ok());
    }
}
