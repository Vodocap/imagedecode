pub struct Bitset
{
    bits: Vec<u64>,
    size: usize,
}

impl Bitset
{
    pub fn new(size: usize) -> Self
    {
        let num_u64 = (size + 63) / 64;
        Bitset
        {
            bits: vec![0; num_u64],
            size,
        }
    }

    pub fn set(&mut self, index: usize)
    {
        if index >= self.size
        {
            panic!("Index out of bounds");
        }
        let u64_index = index / 64;
        let bit_index = index % 64;
        self.bits[u64_index] |= 1 << bit_index;
    }

    pub fn get(&self, index: usize) -> bool
    {
        if index >= self.size
        {
            panic!("index out of bounds");
        }
        let u64_index = index / 64;
        let bit_index = index % 64;
        (self.bits[u64_index] & (1u64 << bit_index)) != 0
    }

    pub fn get_size(&self) -> usize
    {
        self.size
    }

}