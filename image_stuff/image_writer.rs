use image::{GrayImage, Luma};

pub struct ImageWriter;

impl ImageWriter
{
    pub fn new() -> Self
    {
        ImageWriter
    }

    pub fn write_image(&self, filename: &str, bytes: &Vec<u8>) -> Result<(), image::ImageError>
    {

        let len: u32 = bytes.len().try_into().unwrap();
        let width: u32 = (len as f32).sqrt().ceil() as u32;
        let height: u32 = (len + width - 1) / width;


        let mut encoded_img = GrayImage::new(width, height);
        for (i, &byte) in bytes.iter().enumerate()
        {
            let x = (i as u32) % width;
            let y = (i as u32) / width;
            encoded_img.put_pixel(x, y, Luma([byte]));
        }
        
        encoded_img.save(filename)?;
        Ok(())
    }

}

