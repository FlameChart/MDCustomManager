use adw::{AboutDialog, Application, HeaderBar};
use gtk::prelude::*;
use gtk::{
    Align, ApplicationWindow, Box, Button, MenuButton, Orientation, PopoverMenu, Separator, Stack,
    StackSwitcher, StackTransitionType,
};

mod stack_home;
mod stack_mods;
mod stack_songs;
mod stack_tools;

pub struct App {
    pub window: ApplicationWindow,
}

impl App {
    pub fn new(app: &Application) -> Self {
        // Make the stack & switcher working.
        let stack = Stack::builder()
            .halign(Align::Fill)
            .valign(Align::Fill)
            .transition_type(StackTransitionType::SlideLeftRight)
            .transition_duration(800u32)
            .build();

        // Adding and binding pages
        stack.add_titled(&stack_home::create(), Some("home"), "Home");
        stack.add_titled(&stack_mods::create(), Some("mods"), "Mods");
        stack.add_titled(&stack_songs::create(), Some("songs"), "Songs");
        stack.add_titled(&stack_tools::create(), Some("tools"), "Tools");

        // dropdown items
        let menu_settings = Button::builder().label("Settings").build();
        let menu_about = Button::builder().label("About").build();

        // dropdown content box
        let menu_content = Box::builder()
            .margin_top(12i32)
            .margin_bottom(12i32)
            .margin_start(12i32)
            .margin_end(12i32)
            .spacing(6i32)
            .orientation(Orientation::Vertical)
            .build();

        // here arranges all the elements
        menu_content.append(&menu_settings);
        menu_content.append(&Separator::new(Orientation::Horizontal));
        menu_content.append(&menu_about);

        // and here is the main trigger
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .popover(&PopoverMenu::builder().child(&menu_content).build())
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
