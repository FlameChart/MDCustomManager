use gtk::prelude::*;
use adw::Application;

mod app;
use app::App;

fn main() {
    let app = Application::builder()
        .application_id("space.welkin.mdcustommanager")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let app = App::new(app);
    app.window.present();
}