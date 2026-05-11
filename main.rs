#[path = "bits/bitset.rs"]
mod Bitset;


fn main()
{
    println!("Bitset");
    let mut bitset = Bitset::Bitset::new(64);
    bitset.set(10);
    for i in 0..bitset.get_size()
    {
        println!("Bit {}: {}", i, bitset.get(i));
    }
}   