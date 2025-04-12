use gtk::prelude::*;
use gtk::
{
	Application, ApplicationWindow, Button, FileChooserDialog, FileFilter,
};
use std::cell::RefCell;
use std::rc::Rc;
use crate::algo::compression;
use crate::algo::decompression;
use hound;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use image::{Rgb, RgbImage};
use gtk::{Box as GtkBox, Orientation, Image, DrawingArea, Toolbar, ToolButton};
use std::collections::VecDeque;
use gtk::gdk::{EventButton, EventMask, ModifierType};

//link to play_sound
//use super::play_sound::create_audio_player;
//use crate::interface::play_sound::create_audio_player;
use gtk::ProgressBar;
//only for getting path
use std::env;
use std::path::PathBuf;
//use std::path::Path;
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
	let btn_edit = Button::with_label("Edit audio...");
	
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
    use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use gtk::Button;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::rc::Rc;

let btn_play = Button::with_label("Play audio...");
let btn_pause = Button::with_label("Pause audio...");
let window_clone = Rc::clone(&window);
let selected_file_clone = Rc::clone(&selected_file);

let is_playing = Arc::new(AtomicBool::new(false));
let is_paused = Arc::new(AtomicBool::new(false));
let playback_handle = Arc::new(Mutex::new(Option::<(Arc<Mutex<rodio::OutputStreamHandle>>, std::thread::JoinHandle<()>)>::None));

btn_play.connect_clicked({
    let is_playing = Arc::clone(&is_playing);
    let is_paused = Arc::clone(&is_paused);
    let playback_handle = Arc::clone(&playback_handle);
    let window_clone = Rc::clone(&window);
    let selected_file_clone = Rc::clone(&selected_file);

    move |_| {
        if let Some(path) = open_file_dialog(&window_clone, "audio") {
            *selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
            println!("Playing audio file: {} ...", path.display());

            if is_playing.load(Ordering::SeqCst) {
                println!("Audio is already playing.");
            } else {
                is_playing.store(true, Ordering::SeqCst);
                is_paused.store(false, Ordering::SeqCst); // Reset pause state when starting to play

                thread::spawn({
                    let path = path.clone();
                    let is_playing = Arc::clone(&is_playing);
                    let is_paused = Arc::clone(&is_paused);
                    let playback_handle = Arc::clone(&playback_handle);
                    move || {
                        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                        println!("Audio stream initialized.");  // Debugging point

                        let file = File::open(&path).unwrap();
                        let reader = BufReader::new(file);
                        let source = Decoder::new(reader).unwrap();
                        let stream = source.convert_samples::<f32>();

                        // Wrap the stream handle in Arc<Mutex<>> so it can be shared safely
                        let stream_handle = Arc::new(Mutex::new(stream_handle));

                        // Save the stream handle and join handle so we can access them later
                        {
                            let mut handle_lock = playback_handle.lock().unwrap();
                            *handle_lock = Some((Arc::clone(&stream_handle), std::thread::spawn(move || {
                                println!("Starting playback..."); // Debugging point
                                let stream_handle = stream_handle.lock().unwrap();
                                stream_handle.play_raw(stream).unwrap();
                                println!("Audio playback should be ongoing."); // Debugging point
                                loop {
                                    if is_paused.load(Ordering::SeqCst) {
                                        std::thread::sleep(std::time::Duration::from_secs(1)); // Wait if paused
                                    } else if !is_playing.load(Ordering::SeqCst) {
                                        break; // Exit loop when no longer playing
                                    }
                                }
                                println!("Playback finished or stopped.");
                            })));
                        }
                    }
                });
            }
        } else {
            println!("No file selected.");
        }
    }
});

btn_pause.connect_clicked({
    let is_playing = Arc::clone(&is_playing);
    let is_paused = Arc::clone(&is_paused);
    let playback_handle = Arc::clone(&playback_handle);

    move |_| {
        if is_playing.load(Ordering::SeqCst) {
            if is_paused.load(Ordering::SeqCst) {
                // Resume playback
                is_paused.store(false, Ordering::SeqCst);
                println!("Resuming playback...");
            } else {
                // Pause the playback
                is_paused.store(true, Ordering::SeqCst);
                println!("Pausing playback...");
                
                // We don't need to drop the stream handle, we just pause the loop above
            }
        }
    }
});
    
    
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
        	decompression::main(path.to_str().unwrap(), "test_files/output.wav", 1.0);
        	println!("Decompressing file: Done.");
    	} 
    	else 
    	{
        	println!("No file selected.");
    	}
	});
	
	let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);

	//Audio editor
	btn_edit.connect_clicked(move |_| {
    if let Some(path) = open_file_dialog(&window_clone, "audio") {
        *selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        println!("Editing audio file: {} ...", path.display());

        //new window
        let editor_window = ApplicationWindow::builder()
            .application(window_clone.application().as_ref().unwrap())
            .title("Audio Editor")
            .default_width(800)
            .default_height(500)
            .build();

        let vbox_editor = GtkBox::new(Orientation::Vertical, 5);

        //spectrogram
        let image = Image::new();
        let output_img = "test_files/spectrogram.png";
        generate_waveform_image(path.to_str().unwrap(), output_img);
        image.set_from_file(Some(output_img));

        //select zone
        let selection = Rc::new(RefCell::new((None, None)));
        let undo_stack = Rc::new(RefCell::new(VecDeque::new()));
        let redo_stack = Rc::new(RefCell::new(VecDeque::new()));
        let current_audio_path = Rc::new(RefCell::new(path.to_string_lossy().into_owned()));

        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(800, 200);
		drawing_area.set_events(EventMask::BUTTON_PRESS_MASK
        | EventMask::BUTTON_RELEASE_MASK
        | EventMask::POINTER_MOTION_MASK,);
		
        let sel_clone = Rc::clone(&selection);
        drawing_area.connect_button_press_event(move |_, event| {
            let x = event.position().0 as i32;
            sel_clone.borrow_mut().0 = Some(x);
            gtk::glib::Propagation::Proceed
        });
        
        let sel_clone = Rc::clone(&selection);
		drawing_area.connect_motion_notify_event(move |_, event| {
			if event.state().contains(ModifierType::BUTTON1_MASK) {
				let x = event.position().0 as i32;
				sel_clone.borrow_mut().1 = Some(x);
			}
			gtk::glib::Propagation::Proceed
		});

        let sel_clone = Rc::clone(&selection);
        drawing_area.connect_button_release_event(move |_, event| {
            let x = event.position().0 as i32;
            sel_clone.borrow_mut().1 = Some(x);
            gtk::glib::Propagation::Proceed
        });

        //toolbar
        let toolbar = Toolbar::new();
        let btn_cut = ToolButton::new(None::<&gtk::Widget>, Some("✂"));
        let btn_undo = ToolButton::new(None::<&gtk::Widget>, Some("↶"));
		let btn_redo = ToolButton::new(None::<&gtk::Widget>, Some("↷"));

        toolbar.add(&btn_cut);
        toolbar.add(&btn_undo);
        toolbar.add(&btn_redo);

        //cut
        {
            let selection = Rc::clone(&selection);
            let current_audio_path = Rc::clone(&current_audio_path);
            let undo_stack = Rc::clone(&undo_stack);
            let image = image.clone();

            btn_cut.connect_clicked(move |_| {
                let path = current_audio_path.borrow().clone();
                let reader = hound::WavReader::open(&path).expect("open wav failed");
                let samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();
                let sample_rate = 44100;

                let (start_px, end_px) = *selection.borrow();
                if let (Some(mut start), Some(mut end)) = (start_px, end_px) {
                    if start > end {
                        std::mem::swap(&mut start, &mut end);
                    }

                    let start_ratio = start as f32 / 800.0;
                    let end_ratio = end as f32 / 800.0;
                    let start_idx = (start_ratio * samples.len() as f32) as usize;
                    let end_idx = (end_ratio * samples.len() as f32) as usize;

                    undo_stack.borrow_mut().push_back(samples.clone());

                    let mut cut = samples.clone();
                    cut.drain(start_idx..end_idx.min(cut.len()));

                    let tmp_path = "test_files/edited_output.wav";
                    let spec = hound::WavSpec {
                        channels: 1,
                        sample_rate: sample_rate as u32,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };

                    let mut writer = hound::WavWriter::create(tmp_path, spec).unwrap();
                    for s in &cut {
                        writer.write_sample(*s).unwrap();
                    }

                    *current_audio_path.borrow_mut() = tmp_path.to_string();
                    generate_waveform_image(tmp_path, "test_files/spectrogram.png");
                    image.clear();
                    image.set_from_file(Some("test_files/spectrogram.png"));
                    println!("Cut applied.");
                }
            });
        }

        //undo
        {
            let undo_stack = Rc::clone(&undo_stack);
            let redo_stack = Rc::clone(&redo_stack);
            let current_audio_path = Rc::clone(&current_audio_path);
            let image = image.clone();

            btn_undo.connect_clicked(move |_| {
                if let Some(prev) = undo_stack.borrow_mut().pop_back() {
                    redo_stack.borrow_mut().push_back(prev.clone());

                    let tmp_path = "test_files/undo.wav";
                    let spec = hound::WavSpec {
                        channels: 1,
                        sample_rate: 44100,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };
                    let mut writer = hound::WavWriter::create(tmp_path, spec).unwrap();
                    for s in &prev {
                        writer.write_sample(*s).unwrap();
                    }

                    *current_audio_path.borrow_mut() = tmp_path.to_string();
                    generate_waveform_image_from_samples(&prev, "test_files/spectrogram.png");
					image.set_from_file(Some("test_files/spectrogram.png"));
                    println!("Undo applied.");
                }
            });
        }

        //redo
        {
            let undo_stack = Rc::clone(&undo_stack);
            let redo_stack = Rc::clone(&redo_stack);
            let current_audio_path = Rc::clone(&current_audio_path);
            let image = image.clone();

            btn_redo.connect_clicked(move |_| {
                if let Some(next) = redo_stack.borrow_mut().pop_back() {
                    undo_stack.borrow_mut().push_back(next.clone());

                    let tmp_path = "test_files/redo.wav";
                    let spec = hound::WavSpec {
                        channels: 1,
                        sample_rate: 44100,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };
                    let mut writer = hound::WavWriter::create(tmp_path, spec).unwrap();
                    for s in &next {
                        writer.write_sample(*s).unwrap();
                    }

                    *current_audio_path.borrow_mut() = tmp_path.to_string();
                    generate_waveform_image_from_samples(&next, "test_files/spectrogram.png");
					image.clear();
					image.set_from_file(Some("test_files/spectrogram.png"));
                    println!("Redo applied.");
                }
            });
        }

        vbox_editor.pack_start(&toolbar, false, false, 0);
        vbox_editor.pack_start(&drawing_area, false, false, 0);
        vbox_editor.pack_start(&image, true, true, 0);
        editor_window.add(&vbox_editor);
        editor_window.show_all();
    } else {
        println!("No file selected for editing.");
    }
});


    vbox.pack_start(&btn_compress, false, false, 0);
    vbox.pack_start(&btn_decompress, false, false, 0);
    vbox.pack_start(&btn_edit, false, false, 0);
    vbox.pack_start(&btn_play, false, false, 0);
    vbox.pack_start(&btn_pause, false, false, 0);
    window.add(&vbox);
    window.show_all();
    window.present();
}

fn cut_audio(input_path: &str, output_path: &str, start_sec: f32, end_sec: f32) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV");
    let spec = reader.spec();
    let samples: Vec<i16> = reader
        .into_samples::<i16>()
        .filter_map(Result::ok)
        .collect();

    let sample_rate = spec.sample_rate as usize;
    let start_idx = (start_sec * sample_rate as f32) as usize;
    let end_idx = (end_sec * sample_rate as f32) as usize;

    let cut_samples = &samples[start_idx..end_idx.min(samples.len())];

    let writer = hound::WavWriter::create(output_path, spec).expect("Failed to create WAV");
    let mut writer = writer;

    for &s in cut_samples {
        writer.write_sample(s).unwrap();
    }
}

fn generate_waveform_image(input_path: &str, output_path: &str) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV file");
    let samples: Vec<i16> = reader
        .into_samples::<i16>()
        .filter_map(Result::ok)
        .collect();

    generate_waveform_image_from_samples(&samples, output_path);
}

fn generate_waveform_image_from_samples(samples: &Vec<i16>, output_path: &str) {
    let width = 800;
    let height = 200;
    let mut img = RgbImage::new(width, height);

    let samples_per_pixel = samples.len() / (width.max(1) as usize);
    for x in 0..width {
        let start = x as usize * samples_per_pixel;
        let end = ((x + 1) as usize * samples_per_pixel).min(samples.len());
        let slice = &samples[start..end];

        let max = slice.iter().map(|v| v.abs()).max().unwrap_or(0) as f32 / i16::MAX as f32;
        let y = ((1.0 - max) * (height as f32 / 2.0)) as u32;
        for dy in y..(height - y) {
            img.put_pixel(x, dy, Rgb([0, 128, 255]));
        }
    }

    img.save(output_path).expect("Failed to save waveform image");
}

