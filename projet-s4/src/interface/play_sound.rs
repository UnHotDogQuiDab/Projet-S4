use gtk::{glib, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer::prelude::ObjectExt;

static mut PIPELINE: Option<gst::Element> = None;
static mut BUS_WATCH: Option<glib::SourceId> = None;

pub fn create_audio_player(file_path: &str) {
    gst::init().expect("Failed to initialize GStreamer");

    let playbin = gst::ElementFactory::make("playbin")
        .build()
        .expect("Could not build playbin");

    playbin.set_property("uri", &format!("file://{}", file_path));

    playbin.set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    unsafe {
        PIPELINE = Some(playbin.clone());
    }

    let progress_bar = Rc::new(RefCell::new(gtk::ProgressBar::new()));
    let progress_bar_clone = progress_bar.clone();

    thread::spawn(move || {
        loop {
            let position = playbin.query_position::<gst::ClockTime>();
            let duration = playbin.query_duration::<gst::ClockTime>();

            if let (Some(pos), Some(dur)) = (position, duration) {
                let fraction = pos.nseconds() as f64 / dur.nseconds() as f64;
                glib::MainContext::default().invoke(move || {
                    progress_bar_clone.borrow().set_fraction(fraction.clamp(0.0, 1.0));
                });
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
}

pub fn toggle_pause_resume() {
    unsafe {
        if let Some(ref pipeline) = PIPELINE {
            let state = pipeline.current_state();
            match state {
                gst::State::Playing => {
                    pipeline.set_state(gst::State::Paused).unwrap();
                }
                gst::State::Paused | gst::State::Ready | gst::State::Null => {
                    pipeline.set_state(gst::State::Playing).unwrap();
                }
                _ => {}
            }
        }
    }
}

pub fn create_window_with_controls() {
    let application = gtk::Application::new(None, Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Audio Player");
        window.set_default_size(300, 200);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let progress_bar = gtk::ProgressBar::new();
        vbox.pack_start(&progress_bar, true, true, 0);

        let pause_button = gtk::Button::with_label("Pause/Resume");
        vbox.pack_start(&pause_button, false, false, 0);

        window.add(&vbox);
        window.show_all();

        pause_button.connect_clicked(move |_| {
            toggle_pause_resume();
        });

        create_audio_player("path_to_audio_file");
    });

    application.run();
}

fn main() {
    create_window_with_controls();
}
