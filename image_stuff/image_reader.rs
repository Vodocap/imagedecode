
use image::{DynamicImage};

pub struct ImageReader;

impl ImageReader
{
    pub fn new() -> Self
    {
        ImageReader
    }

    pub fn read_image(&self, filename: &str) -> Result<Vec<u8>, std::io::Error>
    {
        let img: DynamicImage = image::open(filename).expect("unable to open image file");
        let rgb_img = img.to_rgb8();
        let bytes = rgb_img.into_raw();

        Ok(bytes)
    }

}

