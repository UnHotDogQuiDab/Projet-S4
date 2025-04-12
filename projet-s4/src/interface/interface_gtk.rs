use gtk::prelude::*;
use gtk::
{
	Application, ApplicationWindow, Button, FileChooserDialog, FileFilter,
};
use std::cell::RefCell;
use std::rc::Rc;
use crate::algo::compression;
use crate::algo::decompression;
use crate::recording::recording;
use crate::volume::volume;
use hound;
use image::{Rgb, RgbImage};
use gtk::{Box as GtkBox, Orientation, Image, DrawingArea, Toolbar, ToolButton};
use std::collections::VecDeque;
use gtk::gdk::{EventMask, ModifierType};

//link to play_sound
//use super::play_sound::create_audio_player;
//use crate::interface::play_sound::create_audio_player;

//only for getting path

//use std::path::Path;



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
    let btn_volume = Button::with_label("Adjust volume...");
    let btn_record = Button::with_label("Record an audio...");
	
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
    let btn_play = Button::with_label("play audio...");
let window_clone = Rc::clone(&window);
let selected_file_clone = Rc::clone(&selected_file);
{
btn_play.connect_clicked(move |_| {

    if let Some(path) = open_file_dialog(&window_clone, "audio") {
        *selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        println!("Playing audio file: {} ...", path.display());

        std::thread::spawn({
            let path = path.clone();
            move || {
                use rodio::{Decoder, OutputStream, Source};
                use std::fs::File;
                use std::io::BufReader;

                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let file = File::open(&path).unwrap();
                let reader = BufReader::new(file);
                let source = Decoder::new(reader).unwrap();
                let stream = source.convert_samples();

                //play
                stream_handle.play_raw(stream).unwrap();

                
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));  
                }
            }
        });

        println!("Playing file: Done.");
    } else {
        println!("No file selected.");
    }
});
}
    
    
    //playsound

    let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);

    //open volume
    btn_volume.connect_clicked(move |_| 
        {
            let dialog = FileChooserDialog::new(
                Some("Select File"),
                Some(&*window_clone),
                gtk::FileChooserAction::Open,
            );
    
            let filter = FileFilter::new();
            filter.add_pattern("*.wav");
            filter.set_name(Some("Wave Files"));
            dialog.add_filter(filter);
    
            dialog.add_buttons(&[
                ("Open", gtk::ResponseType::Accept),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);
        
            let selected_file_clone = selected_file_clone.clone();
            let window_clone_inner = window_clone.clone();
        
            dialog.connect_response(move |file_dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(file) = file_dialog.filename() {
                        *selected_file_clone.borrow_mut() = Some(file.to_string_lossy().into_owned());
                        let path_str = file.to_str().unwrap().to_string();
        
                        file_dialog.close(); 
        
    
                        let speed_dialog = gtk::Dialog::with_buttons(
                            Some("Select Audio Volume"),
                            Some(&*window_clone_inner),
                            gtk::DialogFlags::MODAL,
                            &[("Enter", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
                        );
        
                        let content_area = speed_dialog.content_area();
                        let entry = gtk::Entry::new();
                        entry.set_placeholder_text(Some("Ex: 20.0 for very loud"));
                        content_area.add(&entry);
        
                        speed_dialog.show_all();
        
                        speed_dialog.connect_response(move |dialog, response| {
                            if response == gtk::ResponseType::Ok {
                                let vitesse = entry.text().to_string();
                                if let Ok(vitesse_f) = vitesse.parse::<f32>() {
                                    println!("Adjusting audio: ...");
                                    let _ = volume::adjust_volume(&path_str,"test_files/output.wav", vitesse_f);
                                    println!("Adjust audio: Done.");
                                } else {
                                    println!("No valid entry.");
                                }
                            }
                            dialog.close();
                        });
                    }
                } else {
                    file_dialog.close();
                }
            });
        
            dialog.show_all();
        });
    
        let _window_clone = Rc::clone(&window);
        let _selected_file_clone = Rc::clone(&selected_file);

    //open recording
    btn_record.connect_clicked(move |_| 
        {
            let output_path = "test_files/output.wav";
            let duration_secs = 5;
        
            if let Err(e) = recording::record_wav(output_path, duration_secs) {
                eprintln!("Erreur : {}", e);
            }
            
            });
    
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
	btn_decompress.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Select File"),
            Some(&*window_clone),
            gtk::FileChooserAction::Open,
        );

        let filter = FileFilter::new();
        filter.add_pattern("*.txt");
        filter.set_name(Some("Compressed Files"));
        dialog.add_filter(filter);

        dialog.add_buttons(&[
            ("Open", gtk::ResponseType::Accept),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
    
        let selected_file_clone = selected_file_clone.clone();
        let window_clone_inner = window_clone.clone();
    
        dialog.connect_response(move |file_dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = file_dialog.filename() {
                    *selected_file_clone.borrow_mut() = Some(file.to_string_lossy().into_owned());
                    let path_str = file.to_str().unwrap().to_string();
    
                    file_dialog.close(); 
    

                    let speed_dialog = gtk::Dialog::with_buttons(
                        Some("Set Speed Audio"),
                        Some(&*window_clone_inner),
                        gtk::DialogFlags::MODAL,
                        &[("Enter", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
                    );
    
                    let content_area = speed_dialog.content_area();
                    let entry = gtk::Entry::new();
                    entry.set_placeholder_text(Some("Ex: 2.0 for x2"));
                    content_area.add(&entry);
    
                    speed_dialog.show_all();
    
                    speed_dialog.connect_response(move |dialog, response| {
                        if response == gtk::ResponseType::Ok {
                            let vitesse = entry.text().to_string();
                            if let Ok(vitesse_f) = vitesse.parse::<f64>() {
                                println!("Decompressing file: {} with {}x speed ...", path_str, vitesse_f);
                                decompression::main(&path_str, "test_files/output.wav", vitesse_f);
                                println!("Decompressing file: Done.");
                            } else {
                                println!("No valid entry.");
                            }
                        }
                        dialog.close();
                    });
                }
            } else {
                file_dialog.close();
            }
        });
    
        dialog.show_all();
    });
    
    
	
	let window_clone = Rc::clone(&window);
	let selected_file_clone = Rc::clone(&selected_file);

	//Audio editor
	btn_edit.connect_clicked(move |_| {
    if let Some(path) = open_file_dialog(&window_clone, "audio") {
        *selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        println!("Editing audio file: {} ...", path.display());

        let editor_window = ApplicationWindow::builder()
            .application(window_clone.application().as_ref().unwrap())
            .title("Audio Editor")
            .default_width(800)
            .default_height(500)
            .build();

        let vbox_editor = GtkBox::new(Orientation::Vertical, 5);

        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(800, 200);
        drawing_area.set_events(
            EventMask::BUTTON_PRESS_MASK
                | EventMask::BUTTON_RELEASE_MASK
                | EventMask::POINTER_MOTION_MASK,
        );
		let selection_overlay = Rc::new(RefCell::new(None::<(f64, f64)>));
        let image = Image::new();
        let output_img = "test_files/spectrogram.png";
        generate_waveform_image(path.to_str().unwrap(), output_img);
        image.set_from_file(Some(output_img));

        let undo_stack = Rc::new(RefCell::new(VecDeque::new()));
        let redo_stack = Rc::new(RefCell::new(VecDeque::new()));
        let current_audio_path = Rc::new(RefCell::new(path.to_string_lossy().into_owned()));
		

        //Selection area
		{
			let selection_overlay = Rc::clone(&selection_overlay);
			drawing_area.connect_button_press_event(move |_, event| {
				let x = event.position().0;
				*selection_overlay.borrow_mut() = Some((x, x));
				gtk::glib::Propagation::Proceed
			});
		}
		{
			let selection_overlay = Rc::clone(&selection_overlay);
			drawing_area.connect_motion_notify_event(move |_, event| {
				if event.state().contains(ModifierType::BUTTON1_MASK) {
					let mut overlay = selection_overlay.borrow_mut();
					if let Some((start, _)) = *overlay {
						let x = event.position().0;
						*overlay = Some((start, x));
					}
				}
				gtk::glib::Propagation::Proceed
			});
		}

		{
			let _selection_overlay = Rc::clone(&selection_overlay);
			drawing_area.connect_button_release_event(move |area, _| {
				area.queue_draw(); 
				gtk::glib::Propagation::Proceed
			});
		}
		{
			let selection_overlay = Rc::clone(&selection_overlay);
			drawing_area.connect_draw(move |widget, cr| {

				if let Some((x1, x2)) = *selection_overlay.borrow() {
					let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
					let height = widget.allocated_height() as f64;
					cr.set_source_rgba(0.2, 0.4, 1.0, 0.4); 
					cr.rectangle(start, 0.0, end - start, height);
					cr.fill().unwrap();
				}

				gtk::glib::Propagation::Proceed
			});
		}



        // Toolbar
        let toolbar = Toolbar::new();
        let btn_cut = ToolButton::new(None::<&gtk::Widget>, Some("✂"));
        let btn_undo = ToolButton::new(None::<&gtk::Widget>, Some("↶"));
        let btn_redo = ToolButton::new(None::<&gtk::Widget>, Some("↷"));


        toolbar.add(&btn_cut);
        toolbar.add(&btn_undo);
        toolbar.add(&btn_redo);

        // CUT
        {
            let current_audio_path = Rc::clone(&current_audio_path);
            let undo_stack = Rc::clone(&undo_stack);
            let redo_stack = Rc::clone(&redo_stack);
            let image = image.clone();
            let drawing_area = drawing_area.clone();
			let selection_overlay = Rc::clone(&selection_overlay);

            btn_cut.connect_clicked(move |_| {
                let path = current_audio_path.borrow().clone();
                let reader = hound::WavReader::open(&path).expect("open wav failed");
				let spec = reader.spec();
				let sample_rate = spec.sample_rate;
				let samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();


                let (start_px, end_px) = match *selection_overlay.borrow() {
					Some((x1, x2)) => (Some(x1 as i32), Some(x2 as i32)),
					None => (None, None),
				};

                if let (Some(mut start), Some(mut end)) = (start_px, end_px) {
                    if start > end {
                        std::mem::swap(&mut start, &mut end);
                    }

                    let area_width = drawing_area.allocated_width();
                    let start_idx = (start as f32 / area_width as f32 * samples.len() as f32) as usize;
                    let end_idx = (end as f32 / area_width as f32 * samples.len() as f32) as usize;

                    if start_idx >= end_idx || end_idx > samples.len() {
                        println!("Invalid selection");
                        return;
                    }

                    undo_stack.borrow_mut().push_back(samples.clone());
                    redo_stack.borrow_mut().clear(); // clear redo stack after new edit

                    let mut cut = samples.clone();
                    cut.drain(start_idx..end_idx);

                    let spec = hound::WavSpec {
                        channels: 1,
                        sample_rate,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };

                    let out_path = "test_files/edited_output.wav";
                    let mut writer = hound::WavWriter::create(out_path, spec).unwrap();
                    for s in &cut {
                        writer.write_sample(*s).unwrap();
                    }

                    writer.finalize().unwrap();
                    *current_audio_path.borrow_mut() = out_path.to_string();
					*selection_overlay.borrow_mut() = None;
					drawing_area.queue_draw();
                    generate_waveform_image(out_path, "test_files/spectrogram.png");
                    image.set_from_file(Some("test_files/spectrogram.png"));
                    println!("Cut applied.");
                }
            });
        }

        // UNDO
        {
            let undo_stack = Rc::clone(&undo_stack);
            let redo_stack = Rc::clone(&redo_stack);
            let current_audio_path = Rc::clone(&current_audio_path);
            let image = image.clone();
			let selection_overlay = Rc::clone(&selection_overlay);
			let drawing_area = drawing_area.clone();

            btn_undo.connect_clicked(move |_| {
			let path = current_audio_path.borrow().clone();
			let reader = hound::WavReader::open(&path).expect("open wav failed");
			let current_samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();

			if let Some(prev) = undo_stack.borrow_mut().pop_back() {
				redo_stack.borrow_mut().push_back(current_samples); 

				let spec = hound::WavSpec {
					channels: 1,
					sample_rate: 44100,
					bits_per_sample: 16,
					sample_format: hound::SampleFormat::Int,
				};

				let out_path = "test_files/edited_output.wav";
				let mut writer = hound::WavWriter::create(out_path, spec).unwrap();
				for s in &prev {
					writer.write_sample(*s).unwrap();
				}
				writer.finalize().unwrap();

				*current_audio_path.borrow_mut() = out_path.to_string();
				*selection_overlay.borrow_mut() = None;
				drawing_area.queue_draw();
				generate_waveform_image(out_path, "test_files/spectrogram.png");
				image.set_from_file(Some("test_files/spectrogram.png"));
				image.queue_draw();
				println!("Undo applied.");
			}
		});

        }

        // REDO
        {
            let undo_stack = Rc::clone(&undo_stack);
            let redo_stack = Rc::clone(&redo_stack);
            let current_audio_path = Rc::clone(&current_audio_path);
            let image = image.clone();
			let selection_overlay = Rc::clone(&selection_overlay);
			let drawing_area = drawing_area.clone();

            btn_redo.connect_clicked(move |_| {
				if let Some(next) = redo_stack.borrow_mut().pop_back() {
					undo_stack.borrow_mut().push_back(next.clone());

					let spec = hound::WavSpec {
						channels: 1,
						sample_rate: 44100,
						bits_per_sample: 16,
						sample_format: hound::SampleFormat::Int,
					};

					let out_path = "test_files/edited_output.wav";
					let mut writer = hound::WavWriter::create(out_path, spec).unwrap();
					for s in &next {
						writer.write_sample(*s).unwrap();
					}
					writer.finalize().unwrap();

					*current_audio_path.borrow_mut() = out_path.to_string();
					*selection_overlay.borrow_mut() = None;
					drawing_area.queue_draw();
					generate_waveform_image(out_path, "test_files/spectrogram.png");
					image.set_from_file(Some("test_files/spectrogram.png"));
					image.queue_draw(); 
					println!("Redo applied.");
				}
			});

        }

        vbox_editor.pack_start(&drawing_area, false, false, 0);
        vbox_editor.pack_start(&image, true, true, 0);
        vbox_editor.pack_start(&toolbar, false, false, 0);
        editor_window.add(&vbox_editor);
        editor_window.show_all();
    }
});



    vbox.pack_start(&btn_compress, false, false, 0);
    vbox.pack_start(&btn_decompress, false, false, 0);
    vbox.pack_start(&btn_edit, false, false, 0);
    vbox.pack_start(&btn_play, false, false, 0);
    vbox.pack_start(&btn_record, false, false, 0);
    vbox.pack_start(&btn_volume, false, false, 0);
    window.add(&vbox);
    window.show_all();
    window.present();
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

    let _ = img.save(output_path);//.expect("Failed to save waveform image");
}

