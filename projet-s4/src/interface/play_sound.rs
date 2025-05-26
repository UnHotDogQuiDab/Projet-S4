use gtk::prelude::*;
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

    window.show_all();
    window
}*/
