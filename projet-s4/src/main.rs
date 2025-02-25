mod algo;
use gtk::Application;
use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use projet_s4::interface::interface_gtk::build_interface;

fn main() 
{
	let app = Application::builder()
        .application_id("com.example.interface")
        .build();

    app.connect_activate(|app| build_interface(app));
    app.run();
}
