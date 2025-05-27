use std::fs::File;
use std::io::BufReader;
use crate::algo::compression;
use crate::algo::decompression;
use crate::recording::recording;
use crate::volume::volume;
use gtk::gdk::{EventMask, ModifierType};
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Image, DrawingArea, Toolbar, ToolButton, Overlay, FileChooserAction};
use gtk::{Dialog, DialogFlags, MessageType, ResponseType, Application, ApplicationWindow, Button, FileChooserDialog, FileFilter};
use hound;
use image::{Rgb, RgbImage};
use std::cell::Cell; 
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::ControlFlow::Continue;
use std::path::Path;
use std::rc::Rc;
use std::thread;     
use std::time::Duration; 
use gstreamer as gst;
use gst::prelude::*;
use gstreamer::prelude::ObjectExt;
use glib::clone;
use std::sync::{Arc, Mutex};
use glib::MainContext;

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
        	.title("Audio File Manager")
        	.default_width(400)
        	.default_height(200)
        	.build(),);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

	let btn_edit = Button::with_label("Edit audio file...");
	let btn_decompress = Button::with_label("Edit decompressed audio file...");
    
	
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
		let value = window_clone.clone();
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
                    
                    let app_clone = value.application().unwrap();
					let value = selected_file_clone.clone();
                    speed_dialog.connect_response(move |dialog, response| {
                        if response == gtk::ResponseType::Ok {
                            let vitesse = entry.text().to_string();
                            if let Ok(vitesse_f) = vitesse.parse::<f64>() {
                                println!("Decompressing file: {} with {}x speed ...", path_str, vitesse_f); // Log output
                                decompression::main(&path_str, "test_files/output.wav", vitesse_f);
                                println!("Decompressing file: Done."); 
                                
                                let output_path = Path::new("test_files/output.wav");
								open_editor_window(output_path, &app_clone, Rc::clone(&value));
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
	let value = window_clone.clone();
	btn_edit.connect_clicked(move |_| {
    if let Some(path) = open_file_dialog(&window_clone, "audio") {
        *selected_file_clone.borrow_mut() = Some(path.to_string_lossy().into_owned());
        println!("Editing audio file: {} ...", path.display());
    let app_clone = value.application().unwrap();
    let value = selected_file_clone.clone();
	let output_path = Path::new(&path);
	open_editor_window(output_path, &app_clone, Rc::clone(&value));
    }});


	

    vbox.pack_start(&btn_edit, false, false, 0);
    vbox.pack_start(&btn_decompress, false, false, 0);
    window.add(&vbox);
    window.show_all();
    window.present();
}

fn open_editor_window(path: &Path, app: &Application, selected_file: Rc<RefCell<Option<String>>>) {
    let editor_window = ApplicationWindow::builder()
        .application(app)
        .title("Audio Editor")
        .default_width(800)
        .default_height(200)
        .build();
        
    let vbox_editor = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(800, 200);
	drawing_area.set_app_paintable(true);
    let overlay = Overlay::new();


        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(800, 200);
        drawing_area.set_events(
            EventMask::BUTTON_PRESS_MASK
                | EventMask::BUTTON_RELEASE_MASK
                | EventMask::POINTER_MOTION_MASK,
        );
		let selection_overlay = Rc::new(RefCell::new(None::<(f64, f64)>));
        let image = Image::new();
        overlay.set_child(Some(&image));
        let output_img = "test_files/spectrogram.png";
        generate_waveform_image(path.to_str().unwrap(), output_img);
        image.set_from_file(Some(output_img));

		overlay.add_overlay(&drawing_area);
		
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
		
		//Menubar
		let menu_bar = gtk::MenuBar::new();
		let export_menu_item = gtk::MenuItem::with_label("Export");
		let export_submenu = gtk::Menu::new();

		let export_wav = gtk::MenuItem::with_label("Export as .wav");
		let export_compressed = gtk::MenuItem::with_label("Export as compressed file");

		export_submenu.add(&export_wav);
		export_submenu.add(&export_compressed);
		export_menu_item.set_submenu(Some(&export_submenu));
		menu_bar.add(&export_menu_item);
		
		
/*
{
    let window_clone = Rc::clone(&editor_window);
    let selected_file_clone = Rc::clone(&selected_file);

export_compressed.connect_activate(move |_| {
    if let Some(input_path_str) = selected_file_clone.borrow().as_ref() {
        let input_path = input_path_str.clone();

        // save as
        let save_dialog = FileChooserDialog::with_buttons(
            Some("Save Compressed File"),
            Some(&window_clone),
            FileChooserAction::Save,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Accept),
            ],
        );

        save_dialog.set_do_overwrite_confirmation(true);
        save_dialog.set_current_name("compressed.txt");

        if save_dialog.run() == ResponseType::Accept {
            if let Some(save_path) = save_dialog.get_filename() {
                save_dialog.close();

                // progression
                let progress_dialog = gtk::Dialog::with_buttons(
                    Some("Compression"),
                    Some(&window_clone),
                    DialogFlags::MODAL,
                    &[("Cancel", ResponseType::Cancel)],
                );
                progress_dialog.set_default_size(300, 100);
                let label = gtk::Label::new(Some("Compression in progress..."));
                let content_area = progress_dialog.content_area();
                content_area.add(&label);
                progress_dialog.show_all();

                // cancel
                let cancelled = Rc::new(Cell::new(false));
                let cancelled_clone = cancelled.clone();

                progress_dialog.connect_response(move |dialog, resp| {
                    if resp == ResponseType::Cancel {
                        cancelled_clone.set(true);
                        dialog.close();
                    }
                });

                let window_for_dialog = window_clone.clone();
                let input_path_clone = input_path.clone();
                let save_path_string = save_path.to_string_lossy().to_string();
                let cancelled_for_thread = cancelled.clone();

                thread::spawn(move || {
                    
                    compression::main(&input_path_clone, &save_path_string);

                    // window message
                    glib::idle_add_local(move || {
                        let done_dialog = gtk::MessageDialog::new(
                            Some(&window_for_dialog),
                            DialogFlags::MODAL,
                            MessageType::Info,
                            gtk::ButtonsType::Ok,
                            if cancelled_for_thread.get() {
                                "Compression annulée."
                            } else {
                                "Compression terminée avec succès."
                            },
                        );
                        done_dialog.run();
                        done_dialog.close();
                        Continue(false)
                    });
                });
            } else {
                save_dialog.close();
            }
        } else {
            save_dialog.close();
        }
    } else {

        let msg = gtk::MessageDialog::new(
            Some(&window_clone),
            DialogFlags::MODAL,
            MessageType::Warning,
            gtk::ButtonsType::Ok,
            "No audio file selected to compress.",
        );
        msg.run();
        msg.close();
    }
});

}
*/
        // Toolbar
        let toolbar = Toolbar::new();
        let btn_cut = ToolButton::new(None::<&gtk::Widget>, Some("✂"));
        let btn_undo = ToolButton::new(None::<&gtk::Widget>, Some("↶"));
        let btn_redo = ToolButton::new(None::<&gtk::Widget>, Some("↷"));
		let btn_play = ToolButton::new(None::<&gtk::Widget>, Some("⏯"));
		let btn_volume = ToolButton::new(None::<&gtk::Widget>, Some("🕪"));
		let btn_record = ToolButton::new(None::<&gtk::Widget>, Some("⏺"));	
		
		toolbar.add(&btn_undo);
        toolbar.add(&btn_redo);
        toolbar.add(&btn_cut);
		toolbar.add(&btn_play);
		toolbar.add(&btn_record);
        toolbar.add(&btn_volume);
        
       let btn_open = ToolButton::new(None::<&gtk::Widget>, Some("new file"));
        toolbar.add(&btn_open);
 
        

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
                    redo_stack.borrow_mut().clear(); 

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
 
//playsound

       // PLAY/PAUSE
{
    let current_audio_path = Rc::clone(&current_audio_path);
    let is_playing = Rc::new(RefCell::new(false));
    let current_playbin = Rc::new(RefCell::new(None));

btn_play.connect_clicked(clone!(@strong current_playbin, @strong current_audio_path, @strong is_playing => move |_| {
    let path = current_audio_path.borrow().clone();
    if path.is_empty() {
        println!("No file selected.");
        return;
    }

    let mut playbin_opt = current_playbin.borrow_mut();

    if playbin_opt.is_none() {
        gst::init().expect("Failed to initialize GStreamer");
        let playbin = gst::ElementFactory::make("playbin", None).expect("Could not create playbin");
        playbin.set_property("uri", &format!("file://{}", path));
        playbin.set_state(gst::State::Playing).expect("Failed to set state Playing");

        *playbin_opt = Some(playbin);
        *is_playing.borrow_mut() = true;
        println!("Started new playback");
    } else {
        let playbin = playbin_opt.as_ref().unwrap();

        if *is_playing.borrow() {
            playbin.set_state(gst::State::Paused).expect("Failed to set state Paused");
            *is_playing.borrow_mut() = false;
            println!("Paused");
        } else {
            playbin.set_state(gst::State::Playing).expect("Failed to set state Playing");
            *is_playing.borrow_mut() = true;
            println!("Resumed");
        }
    }
}));
use glib::ControlFlow;
let progress_bar = gtk::Scale::new(gtk::Orientation::Horizontal, None::<&gtk::Adjustment>);
    progress_bar.set_draw_value(false);
    progress_bar.set_range(0.0, 100.0);
    progress_bar.set_value(0.0);

    
    vbox_editor.pack_start(&progress_bar, false, false, 0);

    let current_playbin_for_timeout = Rc::clone(&current_playbin);
let current_playbin_for_slider = Rc::clone(&current_playbin);


let current_playbin_for_close = Rc::clone(&current_playbin);

editor_window.connect_delete_event(move |_, _| {
    if let Some(playbin) = &*current_playbin_for_close.borrow() {
        let _ = playbin.set_state(gst::State::Paused);
        
    }
    false.into() 
});


gtk::glib::timeout_add_local(std::time::Duration::from_millis(200), clone!(@weak progress_bar => @default-return glib::ControlFlow::Continue, move || {
    if let Some(playbin) = &*current_playbin_for_timeout.borrow() {
        if let Some(pos) = playbin.query_position::<gst::ClockTime>() {
            if let Some(dur) = playbin.query_duration::<gst::ClockTime>() {
                let fraction = pos.nseconds() as f64 / dur.nseconds() as f64;
                progress_bar.set_value(fraction * 100.0);
            }
        }
    }
    glib::ControlFlow::Continue
}));

progress_bar.connect_change_value(clone!(@strong current_playbin_for_slider => move |_, _, value| {
    if let Some(playbin) = &*current_playbin_for_slider.borrow() {
        if let Some(duration) = playbin.query_duration::<gst::ClockTime>() {
            let position = (value / 100.0) * duration.nseconds() as f64;
            let seek_result = playbin.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                gst::ClockTime::from_nseconds(position as u64),
            );

            if let Err(err) = seek_result {
                eprintln!("Seek failed: {:?}", err);
            }
        }
    }
    false.into()
}));



let selected_file_clone = Rc::clone(&selected_file);
let current_playbin_clone = Rc::clone(&current_playbin);
let is_playing_clone = Rc::clone(&is_playing);
btn_open.connect_clicked(move |_| {
    let dialog = gtk::FileChooserDialog::new(
        Some("Select Audio File"),
        Some(&gtk::Window::new(gtk::WindowType::Toplevel)), 
        gtk::FileChooserAction::Open,
    );
    dialog.add_buttons(&[
        ("Open", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    dialog.connect_response({
        let selected_file_clone = Rc::clone(&selected_file_clone);
        let current_playbin_clone = Rc::clone(&current_playbin_clone);
        let is_playing_clone = Rc::clone(&is_playing_clone);
        move |dialog, response| {
            if response == gtk::ResponseType::Ok {
                if let Some(path) = dialog.filename() {
                    let uri = format!("file://{}", path.to_string_lossy());
                    *selected_file_clone.borrow_mut() = Some(path.display().to_string());
                    println!("New file selected: {}", path.display());

                    if let Some(ref element) = *current_playbin_clone.borrow() {
    let _ = element.set_state(gstreamer::State::Null);
    let _ = element.set_property("uri", &uri);

    match element.set_state(gstreamer::State::Paused) {
        Ok(success) => {
            println!("Playback started successfully: {:?}", success);
        }
        Err(err) => {
            eprintln!("Failed to start playback: {:?}", err);
        }
    }
    *is_playing_clone.borrow_mut() = false;
    
}
                    else {
                        eprintln!("Playbin is not initialized");
                    }
                }
            }
            dialog.close();
        }
    });

    dialog.show_all();
});

}



//playsound


    // RECORD 
    {
        let selected_file_clone = Rc::clone(&selected_file);
        btn_record.connect_clicked(move |_| {
            if let Some(file_path) = selected_file_clone.borrow().as_ref() {
                let output_path = "test_files/recorded_output.wav";
                let duration_secs = 5;
                println!("Recording audio to {}", output_path); 

                if let Err(e) = recording::record_wav(output_path, duration_secs) {
                    eprintln!("Error while recording : {}", e); 
                }
            } else {
                println!("No file selected."); 
            }
        });
    }

    // VOLUME 
    {
        let selected_file_clone = Rc::clone(&selected_file);
        let current_audio_path = Rc::clone(&current_audio_path);
        let window_clone = editor_window.clone();

        btn_volume.connect_clicked(move |_| {
        if let Some(file_path) = selected_file_clone.borrow().as_ref() {
            let dialog = gtk::Dialog::with_buttons(
                Some("Volume"),
                Some(&window_clone),
                gtk::DialogFlags::MODAL,
                &[("Apply", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );
            let content_area = dialog.content_area();
            let entry = gtk::Entry::new();
            entry.set_placeholder_text(Some("1.0 = normal volume"));
            content_area.add(&entry);
            dialog.show_all();

            let path = file_path.clone();
            let current_audio_path = Rc::clone(&current_audio_path);

            dialog.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    if let Ok(factor) = entry.text().parse::<f32>() {
                        let output_path = "test_files/volume_output.wav";
                        let success = volume::adjust_volume(&path, output_path, factor);
                        
                        if success.is_ok() {
                            *current_audio_path.borrow_mut() = output_path.to_string();
                            println!("Volume adjusted and path updated."); 
                        } else {
                            println!("Error while ajusting volume."); 
                        }
                    } else {
                        println!("Invalid entry."); 
                    }
                }
                dialog.close();
            });
        } else {
            println!("No file selected."); 
        }
    });
    }

		vbox_editor.pack_start(&menu_bar, false, false, 0);
		vbox_editor.pack_start(&toolbar, false, false, 0);
        vbox_editor.pack_start(&overlay, true, true, 0);
        let progress_bar = &vbox_editor.children()[0];
        vbox_editor.reorder_child(progress_bar, 2); 
        editor_window.add(&vbox_editor);
        editor_window.show_all();
}

fn generate_waveform_image(input_path: &str, output_path: &str) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV file");
    let samples: Vec<i16> = reader
        .into_samples::<i16>()
        .filter_map(Result::ok)
        .collect();

    //generate_waveform_image_from_samples(&samples, output_path);
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

    let _ = img.save(output_path);
}
