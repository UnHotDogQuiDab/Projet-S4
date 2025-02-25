mod algo;
use gtk::Application;
use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use projet_s4::interface::interface_gtk::build_interface;
use crate::algo::compression::compression;
use crate::algo::decompression::decompression;

use std::env;

fn main() 
{
	let app = Application::builder()
        .application_id("com.example.interface")
        .build();

    app.connect_activate(|app| build_interface(app));
    app.run();
	
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
