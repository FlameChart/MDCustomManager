use adw::{Application, HeaderBar};
use gtk::prelude::*;
use gtk::{Align, ApplicationWindow, Label, Stack, StackSwitcher, StackTransitionType};

mod stack_home;
mod stack_mods;
mod stack_songs;
mod stack_tools;

pub struct App {
    pub window: ApplicationWindow,
}

impl App {
    pub fn new(app: &Application) -> Self {
        // Hello World Label
        let hello_world_label = Label::builder()
            .label("Hello, World!")
            .margin_top(12i32)
            .margin_bottom(12i32)
            .margin_start(12i32)
            .margin_end(12i32)
            .build();

        // Vertical Box Container in the main window
        let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
        content.append(&hello_world_label);

        // Make the stack / switcher working.
        let stack = Stack::builder()
            .margin_top(12i32)
            .margin_end(12i32)
            .margin_start(12i32)
            .margin_bottom(12i32)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .transition_type(StackTransitionType::SlideLeftRight)
            .transition_duration(800u32)
            .build();

        stack.add_titled(&content, Some("home"), "Home");
        stack.add_titled(&content, Some("mods"), "Mods");
        stack.add_titled(&content, Some("songs"), "Songs");
        stack.add_titled(&content, Some("tools"), "Tools");

        // The Custom HeaderBar with Switcher (but the stack is still missing)
        let header_bar = HeaderBar::builder()
            .title_widget(&StackSwitcher::builder().stack(&stack).build())
            .build();

        // Application Window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("MuseDash Custom Manager")
            .titlebar(&header_bar)
            .default_width(900)
            .default_height(640)
            .child(&stack)
            .build();

        Self { window }
    }
}
