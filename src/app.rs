use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};
use adw::{Application, HeaderBar, ViewSwitcher};

pub struct App {
    pub window: ApplicationWindow,
}

impl App {
    pub fn new(app: &Application) -> Self {
        // Hello World Label
        let hello_world_label = Label::builder()
            .label("Hello, World!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        
        // TODO: Make the stack working.
        
        // The Custom HeaderBar with Switcher (but the stack is still missing)
        let header_bar = HeaderBar::new();
        header_bar.set_title_widget(Some(&ViewSwitcher::new()));

        // Vertical Box Container in the main window
        let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
        content.append(&hello_world_label);

        // Application Window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("MuseDash Custom Manager")
            .titlebar(&header_bar)
            .default_width(900)
            .default_height(640)
            .child(&content)
            .build();

        Self { window }
    }
}