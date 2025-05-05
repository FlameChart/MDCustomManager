use adw::prelude::*;
use gtk::{Align, Box, Label, Orientation};

pub fn create() -> Box {
    let label = Label::builder()
        .label("Home")
        .build();

    // main content
    let content = Box::builder()
        .margin_top(16i32)
        .margin_bottom(16i32)
        .margin_start(16i32)
        .margin_end(16i32)
        .orientation(Orientation::Vertical)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build();
    content.append(&label);

    content
}
