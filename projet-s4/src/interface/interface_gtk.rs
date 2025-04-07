use gtk::prelude::*;
use gtk::
{
	Application, ApplicationWindow, Button, FileChooserDialog, FileFilter,
};
use std::cell::RefCell;
use std::rc::Rc;
use crate::algo::compression;
use crate::algo::decompression;

//link to play_sound
//use super::play_sound::create_audio_player;
//use crate::interface::play_sound::create_audio_player;
use gtk::ProgressBar;
//only for getting path
use std::env;
use std::path::PathBuf;
use std::path::Path;
fn get_current_directory() -> PathBuf 
{
    env::current_dir().expect("Failed to get current directory")
}
fn get_compressed_file_path() -> PathBuf {
    //get_current_directory().join("src/test_files/compressed.txt")
     Path::new("src/test_files/compressed.txt").to_path_buf()
}

fn get_decompressed_file_path() -> PathBuf {
   // get_current_directory().join("src/test_files/output.wav")
    Path::new("src/test_files/output.wav").to_path_buf()
}



//end of getting path



pub fn build_interface(app: &Application) 
{
    let window = Rc::new(ApplicationWindow::builder()
        	.application(app)
        	.title("Audio Compressor/Decompressor")
        	.default_width(400)
        	.default_height(200)
        	.build(),);


    


    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let btn_compress = Button::with_label("Compress audio file...");
    let btn_decompress = Button::with_label("Decompress audio file...");

    let selected_file = Rc::new(RefCell::new(None));

    let open_file_dialog = |window: &ApplicationWindow, file_type: &str| 
	{
        let dialog = FileChooserDialog::new(
            Some("Select File"),
            Some(window),
            gtk::FileChooserAction::Open,
        );

        //filters
        let filter = FileFilter::new();
        if file_type == "audio" 
		{
            filter.add_mime_type("audio/*");
            filter.set_name(Some("Audio Files"));
        }
            else 
            {
                if file_type == "compressed"
            {
                filter.add_pattern("*.txt");
            

            filter.set_name(Some("Compressed Files"));
            }
            }
        
        dialog.add_filter(filter);

        dialog.add_buttons(&[
            ("Open", gtk::ResponseType::Accept),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);

        let result = if dialog.run() == gtk::ResponseType::Accept 
		{
            dialog.file().and_then(|f| f.path())
        } 
		else 
		{
            None
        };
        dialog.close();
        result
    };



    //playsound
    
   

    let play_button = Button::with_label("Play Sound");

play_button.connect_clicked(move |_| {
    let audio_window = ApplicationWindow::builder()
        .title("Audio Player")
        .default_width(600)
        .default_height(400)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let select_button = Button::with_label("Select and Play Audio");

    let progress_bar = ProgressBar::new();
        //gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
    progress_bar.set_draw_value(false);
    progress_bar.set_visible(false);

    let pause_button = Button::with_label("Pause/Resume");
    pause_button.set_visible(false);

    vbox.pack_start(&select_button, false, false, 0);
    vbox.pack_start(&progress_bar, false, false, 0);
    vbox.pack_start(&pause_button, false, false, 0);
    audio_window.set_child(Some(&vbox));

    select_button.connect_clicked({
        let progress_bar = progress_bar.clone();
        let pause_button = pause_button.clone();
        let audio_window = audio_window.clone();
        move |_| {
            let dialog = FileChooserDialog::new(
                Some("Choose Audio File"),
                Some(&audio_window),
                gtk::FileChooserAction::Open,
            );

            let filter = FileFilter::new();
            filter.add_mime_type("audio/*");
            filter.set_name(Some("Audio Files"));
            dialog.add_filter(filter);

            dialog.add_buttons(&[
                ("Open", gtk::ResponseType::Accept),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);

            if dialog.run() == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file().and_then(|f| f.path()) {
                    println!("Playing: {}", file.display());

                    crate::interface::play_sound::create_audio_player(
                        file.to_str().unwrap(),
                        &progress_bar,
                    );

                    progress_bar.set_visible(true);
                    pause_button.set_visible(true);
                }
            }
            dialog.close();
        }
    });

    pause_button.connect_clicked(move |_| {
        crate::interface::play_sound::toggle_pause_resume(&progress_bar);
    });

    audio_window.show();
});

vbox.pack_start(&play_button, false, false, 0);
    
    //playsound

    let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);



    //open file to compress
    btn_compress.connect_clicked(move |_| 
    {
    	if let Some(path) = open_file_dialog(&window_clone, "audio") 
    	{
        	*selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        	println!("Compressing file: {} ...", path.display());
        	compression::main(path.to_str().unwrap(), "test_files/compressed.txt");
            //compression::main(path.to_str().unwrap(), get_compressed_file_path().to_str().unwrap()); 
        	println!("Compressing file: Done.");	
    	} 
    	else 
    	{
        	println!("No file selected.");
    	}
	});

	let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);

	//open file to decompress
	btn_decompress.connect_clicked(move |_| 
	{
    	if let Some(path) = open_file_dialog(&window_clone, "compressed") 
    	{
        	*selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        	println!("Decompressing file: {} ...", path.display());
        	decompression::main(path.to_str().unwrap(), "test_files/output.wav");
            //decompression::main(path.to_str().unwrap(), get_decompressed_file_path().to_str().unwrap());
        	println!("Decompressing file: Done.");
    	} 
    	else 
    	{
        	println!("No file selected.");
    	}
	});


    vbox.pack_start(&btn_compress, false, false, 0);
    vbox.pack_start(&btn_decompress, false, false, 0);
    window.add(&vbox);
    window.show_all();
    window.present();
}
