use crate::bytes_extractor::BytesExtractor;
use crate::image_reader::ImageReader;
use crate::image_writer::ImageWriter;
use crate::reconstructor::Reconstructor;

#[path = "file_reconstruction/reconstructor.rs"]
mod reconstructor;

#[path = "bytes/bytes_extractor.rs"]
mod bytes_extractor;

#[path = "image_stuff/image_reader.rs"]
mod image_reader;

#[path = "image_stuff/image_writer.rs"]
mod image_writer;

fn main()
{
    let bytes_extractor = BytesExtractor::new(); 
    let bytes_result = BytesExtractor::extract(&bytes_extractor, "adresy.zip");

    let bytes = match bytes_result
    {
        Ok(b) => b,
        Err(e) => { print!("Chyba pri načítaní: {}", e);
        return;
    }
    };

    let image_writer = ImageWriter::new();
    let vysledok: Result<(), image::ImageError> = image_writer.write_image("obrastek.png", &bytes);

    print!("=========================================== OBRAZOK PO DEKODOVANI ===========================================");
    
    vysledok.expect("Failed to write image: Je koniec");

    let image_reader = ImageReader::new();
    let image_result = ImageReader::read_image(&image_reader,"obrastek.png");

    let image_bytes = match image_result
    {
        Ok(b) => b,
        Err(e) => 
        {
            print!("Chyba pri nacitani obraska {}", e);
            return;
        }

    };

    let reconstructor = Reconstructor::new();
    reconstructor.reconstruct("recunstructed_file", &image_bytes);



}   