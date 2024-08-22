use std::fs::{File, remove_file, rename};
use std::io::{BufWriter, BufReader};
use png::{Decoder, Encoder, ColorType, Compression};
use log::{info, error};
use clap::ArgMatches;

pub struct Config {
    pub infile: String,
    pub outfile: Option<String>,
    pub inplace: bool,
    pub verbose: u8,
    pub chunk_size: usize,
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        Config {
            infile: matches.get_one::<String>("input").unwrap().to_string(),
            outfile: matches.get_one::<String>("output").map(|s| s.to_string()),
            inplace: matches.get_flag("inplace"),
            verbose: matches.get_count("verbose") as u8,
            chunk_size: *matches.get_one::<usize>("chunk_size").unwrap(),
        }
    }
}

pub fn crush(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = &config.infile;
    let output_path = config.outfile.as_deref().unwrap_or("output.png");

    // Open input PNG file
    let input_file = File::open(input_path).map_err(|e| {
        error!("Failed to open input file: {}", e);
        e
    })?;
    let decoder = Decoder::new(BufReader::new(input_file));

    // Read the PNG info
    let mut reader = decoder.read_info().map_err(|e| {
        error!("Failed to read PNG info: {}", e);
        e
    })?;

    // Clone the relevant image information to avoid borrowing issues
    let (width, height, color_type) = (reader.info().width, reader.info().height, reader.info().color_type);

    // Ensure the image is RGBA
    if color_type != ColorType::Rgba {
        error!("Expected an RGBA image but found {:?}", color_type);
        return Err("Expected an RGBA image".into());
    }

    // Allocate the buffer for the image
    let mut buf = vec![0; reader.output_buffer_size()];

    // Read the image data into the buffer
    reader.next_frame(&mut buf).map_err(|e| {
        error!("Failed to read PNG frame: {}", e);
        e
    })?;

    // Verify buffer size matches expected size
    let expected_size = width as usize * height as usize * 4;
    if buf.len() != expected_size {
        error!("Wrong data size, expected {} but got {}", expected_size, buf.len());
        return Err(format!(
            "wrong data size, expected {} got {}",
            expected_size, buf.len()
        ).into());
    }

    // Apply RGBA -> BGRA transformation with alpha premultiplication
    buf.chunks_exact_mut(4).for_each(|chunk| {
        let (r, g, b, a) = (chunk[0], chunk[1], chunk[2], chunk[3]);
        chunk[0] = (b as u16 * a as u16 / 255) as u8;
        chunk[1] = (g as u16 * a as u16 / 255) as u8;
        chunk[2] = (r as u16 * a as u16 / 255) as u8;
    });

    // Create output PNG file
    let output_file = File::create(output_path).map_err(|e| {
        error!("Failed to create output file: {}", e);
        e
    })?;
    let writer = BufWriter::new(output_file);

    // Setup PNG encoder
    let mut encoder = Encoder::new(writer, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_compression(Compression::Best);

    let mut png_writer = encoder.write_header().map_err(|e| {
        error!("Failed to write PNG header: {}", e);
        e
    })?;

    // Write processed image data to output file
    png_writer.write_image_data(&buf).map_err(|e| {
        error!("Failed to write PNG data: {}", e);
        e
    })?;

    // Handle in-place replacement if needed
    if config.inplace {
        remove_file(input_path).map_err(|e| {
            error!("Failed to remove input file: {}", e);
            e
        })?;
        rename(output_path, input_path).map_err(|e| {
            error!("Failed to rename file: {}", e);
            e
        })?;
    }

    info!("Processed image successfully");
    Ok(())
}
