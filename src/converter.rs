use std::{fs::OpenOptions, io::Write, path::Path};

use crate::error::ConverterResult;
use image::{imageops::FilterType, io::Reader as ImageReader, ImageBuffer, Luma};

pub fn convert(args: crate::ProcessedArgs) -> ConverterResult<()> {
    let reader = ImageReader::open(&args.file)?;
    let mut img = reader.decode()?;
    if args.ratio {
        img = img.resize(args.width, args.height, FilterType::Lanczos3);
    } else {
        img = img.resize_exact(args.width, args.height, FilterType::Lanczos3);
    }
    let img = img.to_luma8();
    let s = luma8_img_to_ascii_string(img, args.width as usize, args.height as usize, args.inv);
    write_string_to_path(&s, args.out)?;
    Ok(())
}

fn write_string_to_path(s: &str, path: impl AsRef<Path>) -> ConverterResult<()> {
    let mut opener = OpenOptions::new();
    opener.write(true).create(true).truncate(true);
    let mut file = opener.open(path)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

const CHARS: &[char] = &['.', '\'', '\"', '/', '#', '@'];

fn luma8_to_ascii_char(luma: &Luma<u8>, inv: bool, level_len: usize) -> char {
    let mut n = luma.0[0];
    if !inv {
        n = u8::MAX - n;
    }
    let index = n as usize / level_len;
    CHARS[index]
}

fn luma8_img_to_ascii_string(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    width: usize,
    height: usize,
    inv: bool,
) -> String {
    let mut out = String::with_capacity((width + 1) * height);
    let level_len = ((u8::MAX as f64) / (CHARS.len() as f64)).ceil() as usize + 1;
    for row in img.rows() {
        for pixel in row {
            let c = luma8_to_ascii_char(pixel, inv, level_len);
            out.push(c);
        }
        out.push('\n')
    }
    out
}
