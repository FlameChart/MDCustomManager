use adw::{Application, HeaderBar};
use gtk::prelude::*;
use gtk::{Align, ApplicationWindow, Box, Button, Label, MenuButton, Orientation, PopoverMenu, Separator, Stack, StackSwitcher, StackTransitionType};

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
        let content = Box::new(Orientation::Vertical, 8);
        content.append(&hello_world_label);

        // Make the stack & switcher working.
        let stack = Stack::builder()
            .halign(Align::Fill)
            .valign(Align::Fill)
            .transition_type(StackTransitionType::SlideLeftRight)
            .transition_duration(800u32)
            .build();

        stack.add_titled(&content, Some("home"), "Home");
        stack.add_titled(&content, Some("mods"), "Mods");
        stack.add_titled(&content, Some("songs"), "Songs");
        stack.add_titled(&content, Some("tools"), "Tools");

        // dropdown items
        let menu_about = Button::builder()
            .label("About")
            // .icon_name("help-about-symbolic")
            .build();
        
        let menu_settings = Button::builder()
            .label("Settings")
            // .icon_name("preferences-system-symbolic")
            .build();
        
        let menu_label = Label::builder()
            .label("Muse Dash Custom Manager")
            .build();
        

        // dropdown content box
        let menu_content = Box::builder()
            .margin_top(12i32)
            .margin_bottom(12i32)
            .margin_start(12i32)
            .margin_end(12i32)
            .spacing(6i32)
            .orientation(Orientation::Vertical)
            .build();
        menu_content.append(&menu_settings);
        menu_content.append(&Separator::new(Orientation::Horizontal));
        menu_content.append(&menu_about);
        menu_content.append(&menu_label);

        // Creates the dropdown (or popover) menu
        let menu_popover = PopoverMenu::builder()
            .child(&menu_content)
            .build();

        // and here is the main trigger
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .popover(&menu_popover)
            .build();

        // The Custom HeaderBar with Switcher (but the stack is still missing)
        let header_bar = HeaderBar::builder()
            .title_widget(&StackSwitcher::builder().stack(&stack).build())
            .build();
        header_bar.pack_start(&menu_button);

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
