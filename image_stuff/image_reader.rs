
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
        let luma_img = img.to_luma8();
        let bytes = luma_img.into_raw();

        Ok(bytes)
    }

}

