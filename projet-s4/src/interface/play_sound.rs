/*use gtk::prelude::*;
use gtk::{Button, FileChooserAction, FileChooserDialog, Orientation, ResponseType, Window, WindowType};
use gstreamer as gst;
use gst::prelude::*;
use gstreamer::glib::ObjectExt;
use std::rc::Rc;
use std::cell::RefCell;

/*pub fn spawn_audio_player_window(parent: &gtk::ApplicationWindow, _file_path: &str) -> gtk::Window {
    // File chooser to select a .wav file
    let file_dialog = FileChooserDialog::new(
        Some("Select WAV file"),
        Some(parent),
        FileChooserAction::Open,
    );
    file_dialog.add_buttons(&[
        ("Open", ResponseType::Accept),
        ("Cancel", ResponseType::Cancel),
    ]);

    let selected_path = if file_dialog.run() == ResponseType::Accept {
        file_dialog.filename()
    } else {
        file_dialog.close();
        return Window::new(WindowType::Toplevel);
    };
    file_dialog.close();

    let filepath = selected_path.unwrap();
    let uri = format!("file://{}", filepath.to_str().unwrap());

    // GStreamer player
    gst::init().expect("Failed to initialize GStreamer");
    let playbin = gst::ElementFactory::make("playbin", None).expect("Could not create playbin");
    playbin.set_property("uri", &uri);

    // GTK window for controls
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Audio Player");
    window.set_default_size(300, 100);

    let vbox = gtk::Box::new(Orientation::Vertical, 5);


    let play_pause_button = Button::with_label("Play");
//let stop_button = Button::with_label("Stop");

vbox.add(&play_pause_button);
//vbox.add(&stop_button);

let playbin_clone = playbin.clone();
let play_pause_button_clone = play_pause_button.clone();
let is_playing = Rc::new(RefCell::new(false));

play_pause_button.connect_clicked(move |_| {
    let mut is_playing_mut = is_playing.borrow_mut();
    if *is_playing_mut {
        playbin_clone.set_state(gst::State::Paused).expect("Failed to pause");
        play_pause_button_clone.set_label("Play");
        
    } else {
        playbin_clone.set_state(gst::State::Playing).expect("Failed to play");
        play_pause_button_clone.set_label("Pause");
        
    }
    *is_playing_mut = !*is_playing_mut;
});

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use glib::clone;
use glib::ControlFlow;
use gstreamer::prelude::Cast;

let is_seeking = Rc::new(Cell::new(false));

let adjustment = gtk::Adjustment::new(0.0, 0.0, 1.0, 0.01, 0.1, 0.0);
let progress_bar = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
progress_bar.set_draw_value(false);
vbox.add(&progress_bar);

let playbin_clone = playbin.clone();
//let playbin_clone2 = playbin.clone();

// Fonction pour fixer la durée max, à appeler au bon moment
let set_duration = || {
    if let Some(duration) = playbin_clone.query_duration::<gst::ClockTime>() {
        let ns = duration.nseconds();
        if ns > 0 {
            adjustment.set_upper(ns as f64);
        }
    }
};
set_duration();

// Gestion des clics pour détecter quand l'utilisateur manipule la barre
{
    let is_seeking = is_seeking.clone();
    progress_bar.connect_button_press_event(move |_, _| {
        is_seeking.set(true);
        false.into()
    });
}

{
    let is_seeking = is_seeking.clone();
    let playbin_clone = playbin_clone.clone();
    let adjustment = adjustment.clone();
    progress_bar.connect_button_release_event(move |scale, _| {
        let value = scale.value();

        playbin_clone.seek_simple(
            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
            gst::ClockTime::from_nseconds(value as u64),
        ).expect("Failed to seek");

        is_seeking.set(false);
        false.into()
    });
}

// Timer de mise à jour régulière
glib::timeout_add_local(
    Duration::from_millis(100),
    clone!(@weak progress_bar, @strong playbin_clone, @strong adjustment, @strong is_seeking => @default-return ControlFlow::Continue, move || {
        if !is_seeking.get() {
            if let Some(position) = playbin_clone.query_position::<gst::ClockTime>() {
                let ns = position.nseconds();
                if (adjustment.value() - ns as f64).abs() > 10.0 {
                    adjustment.set_value(ns as f64);
                }
            }
        }
        ControlFlow::Continue
    }),
);

/*
// Optionnel : pour fixer la durée quand elle change,
// écoute les messages du bus GStreamer
use gstreamer::prelude::Continue;
let playbin_clone2 = playbin_clone2.clone();
let adjustment = adjustment.clone();

let set_duration = move || {
    if let Some(duration) = playbin_clone2.query_duration::<gst::ClockTime>() {
        let ns = duration.nseconds();
        adjustment.set_upper(ns as f64);
    }
};

let bus = playbin_clone.bus().expect("Failed to get bus from playbin");
let playbin_for_bus = playbin_clone.clone();

bus.add_watch_local(move |_, msg| {
    use gst::MessageView;

    match msg.view() {
        gst::MessageView::StateChanged(s) => {
            if s.src().map(|s| s == *playbin_for_bus.upcast_ref::<gst::Element>()).unwrap_or(false) {
                set_duration();
            }
        }
        _ => (),
    }

   Continue(true) 
}).expect("Failed to add bus watch");

    window.add(&vbox);

    // Connect button signals
    /*let playbin_clone = playbin.clone();
    play_pause_button.connect_clicked(move |_| {
        playbin_clone.set_state(gst::State::Playing).expect("Unable to play");
    });*/

    /*let playbin_clone = playbin.clone();
    pause_button.connect_clicked(move |_| {
        playbin_clone.set_state(gst::State::Paused).expect("Unable to pause");
    });*/

   /* stop_button.connect_clicked(move |_| {
        playbin.set_state(gst::State::Null).expect("Unable to stop");
    });*/

let playbin_clone = playbin.clone();
window.connect_delete_event(move |_, _| {
    playbin_clone.set_state(gst::State::Null).expect("Failed to stop playback on close");
    false.into()
});
*/
    window.show_all();
    window
}*/
