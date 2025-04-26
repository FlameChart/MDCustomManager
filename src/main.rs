use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};
use adw::Application;

fn main() {
    let app = Application::builder()
        .application_id("space.welkin.mdcustommanager")
        .build();

    // 当应用程序被激活时执行的回调
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    // 创建一个标签控件
    let label = Label::builder()
        .label("Hello, World!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // 创建一个垂直的盒子容器
    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
    content.append(&label);

    // 创建应用程序窗口
    let window = ApplicationWindow::builder()
        .application(app)
        .title("MuseDash Custom Manager")
        .default_width(800)
        .default_height(480)
        .child(&content)
        .build();

    // 显示窗口
    window.present();
}