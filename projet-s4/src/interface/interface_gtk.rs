use gtk::prelude::*;
use gtk::
{
	Application, ApplicationWindow, Button, FileChooserDialog, FileFilter,
};
use std::cell::RefCell;
use std::rc::Rc;

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
		else if file_type == "compressed" 
		{
            filter.add_pattern("*.zip");
            filter.add_pattern("*.rar");
            filter.add_pattern("*.tar.gz");
            filter.set_name(Some("Compressed Files"));
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
    
    let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);

    //open file to compress
    btn_compress.connect_clicked(move |_| 
    {
    	if let Some(path) = open_file_dialog(&window_clone, "audio") 
    	{
        	*selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        	println!("Compressing file: {} ...", path.display());
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
