mod converter;
mod error;

use crate::error::ConverterResult;
use image::io::Reader as ImageReader;
use std::convert::TryFrom;

/// Arguments for img2ascii
#[derive(argh::FromArgs)]
struct Args {
    /// path to the image file
    #[argh(option, short = 'f')]
    file: String,
    /// optional - width of the ascii output in characters.
    #[argh(option, short = 'w')]
    width: Option<u32>,
    /// optional - height of the ascii output in rows of characters.
    #[argh(option, short = 'h')]
    height: Option<u32>,
    /// optional - path to the ascii output file. default location is ./ascii-out.txt
    #[argh(option, short = 'o')]
    out: Option<String>,
    /// preserve the aspect ratio of the image when resizing
    #[argh(switch, short = 'r')]
    ratio: bool,
    /// turn the image into ascii text with the color values inverted
    #[argh(switch)]
    inv: bool,
}

pub struct ProcessedArgs {
    pub file: String,
    pub width: u32,
    pub height: u32,
    pub out: String,
    pub ratio: bool,
    pub inv: bool,
}

impl TryFrom<Args> for ProcessedArgs {
    type Error = crate::error::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let (mut width, mut height);
        let mut out = "ascii-out.txt".to_string();
        if args.width.is_none() || args.height.is_none() {
            let dims = image_dimensions(&args.file)?;
            width = dims.0;
            height = dims.1;
            width = args.width.unwrap_or(width);
            height = args.height.unwrap_or(height);
        } else {
            width = args.width.unwrap();
            height = args.height.unwrap();
        }
        if args.out.is_some() {
            out = args.out.unwrap();
        }
        Ok(Self {
            file: args.file,
            width,
            height,
            out,
            ratio: args.ratio,
            inv: args.inv,
        })
    }
}

/// Returns the dimensions of an image
fn image_dimensions(path: &str) -> ConverterResult<(u32, u32)> {
    let reader = ImageReader::open(path)?;
    Ok(reader.into_dimensions()?)
}

fn main() {
    let args = ProcessedArgs::try_from(argh::from_env::<Args>()).unwrap();
    crate::converter::covert(args).unwrap();
}
