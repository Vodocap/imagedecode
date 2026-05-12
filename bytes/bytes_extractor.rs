use std::fs::File;
use std::io::Read;


pub struct BytesExtractor
{

}

impl BytesExtractor
{
    pub fn new() -> Self
    {
        BytesExtractor {}
    }

    pub fn extract(&self, filename: &str) -> Result<Vec<u8>, std::io::Error>
    {
        let mut file = File::open(filename).expect("unable to open file");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}
