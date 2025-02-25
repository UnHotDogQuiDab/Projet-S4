mod algo;

use crate::algo::compression::compression;
use crate::algo::decompression::decompression;
use std::env;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 
    {
        println!("Needs more args");
        return;
    }

    let input_file = &args[1];
    let compressed_file = &args[2];
    let output_file = &args[3];

    compression(input_file, compressed_file);
    println!("\n\n--------------------OBALIIIIIIIIE!!!!!!! (ça a compressé)--------------------\n\n");
    decompression(compressed_file, output_file);
    println!("\n\n--------------------TOGEPIIIIIIII!!!!!!! (ça a décompressé)--------------------\n\n");
}
