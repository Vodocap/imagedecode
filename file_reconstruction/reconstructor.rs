use mimetype_detector;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct Reconstructor
{

}

impl Reconstructor
{
    pub fn new() -> Self
    {
        Reconstructor {}
    }

    pub fn reconstruct(&self, filename_no_type: &str, bytes: &Vec<u8>) 
    {
        let file = File::create(filename_no_type).expect("unable to open file");

        BufWriter::new(file).write_all(&bytes).expect("zle je ");

        let mime_type: &mimetype_detector::MimeType = mimetype_detector::detect_file(filename_no_type).expect("Zle je ");
        
        println!("{}", mime_type.extension());
        std::fs::rename(filename_no_type, filename_no_type.to_owned() + mime_type.extension()).expect("Velmi zle je");

    }
}
