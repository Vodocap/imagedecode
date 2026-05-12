use crate::bytes_extractor::BytesExtractor;
use crate::image_reader::ImageReader;


#[path = "bytes/bytes_extractor.rs"]
mod bytes_extractor;

#[path = "image_stuff/image_reader.rs"]
mod image_reader;


fn main()
{
    let bytes_extractor = BytesExtractor::new(); 
    let bytes_result = BytesExtractor::extract(&bytes_extractor, "main.rs");

    let bytes = match bytes_result
    {
        Ok(b) => b,
        Err(e) => { print!("Chyba pri načítaní: {}", e);
        return;
    }
    };

    for (index, &byte) in bytes.iter().enumerate()
    {
        print!("Bajt {}: {} (hex: 0x{:02x}) ", index, byte, byte);
    }


    let image_reader = ImageReader::new();
    let image_result = ImageReader::read_image(&image_reader,"tuxik.png");

    let image_bytes = match image_result
    {
        Ok(b) => b,
        Err(e) => 
        {
            print!("Chyba pri nacitani obraska {}", e);
            return;
        }

    };

    for (index, &image_byte) in image_bytes.iter().enumerate()
    {
        print!("Bajt {}: {} (hex: 0x{:02x}) ", index, image_byte, image_byte);
    }


}   