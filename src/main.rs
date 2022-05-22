use gtk4 as gtk;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.aprilJade.rs-calculator"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    // Create a new window, set its title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Calculator"));

    // Here we construct the grid that is going contain our buttons.
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(80)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(0)
        .column_spacing(0)
        .can_focus(false)
        .build();
    window.set_child(Some(&grid));

    let btn_len = 1;
    let button_0 = gtk::Button::with_label("0");
    let button_1 = gtk::Button::with_label("1");
    let button_2 = gtk::Button::with_label("2");
    let button_3 = gtk::Button::with_label("3");
    let button_4 = gtk::Button::with_label("4");
    let button_5 = gtk::Button::with_label("5");
    let button_6 = gtk::Button::with_label("6");
    let button_7 = gtk::Button::with_label("7");
    let button_8 = gtk::Button::with_label("8");
    let button_9 = gtk::Button::with_label("9");
    let button_cancel = gtk::Button::with_label("AC");
    let button_sign = gtk::Button::with_label("+/-");
    let button_modular = gtk::Button::with_label("%");
    let button_divider = gtk::Button::with_label("÷");
    let button_mutiplier = gtk::Button::with_label("x");
    let button_minus = gtk::Button::with_label("-");
    let button_plus = gtk::Button::with_label("+");
    let button_equal = gtk::Button::with_label("=");
    let button_point = gtk::Button::with_label(".");

    grid.attach(&button_cancel, 0, 0, btn_len, btn_len);
    grid.attach(&button_sign, 1, 0, btn_len, btn_len);
    grid.attach(&button_modular, 2, 0, btn_len, btn_len);
    grid.attach(&button_divider, 3, 0, btn_len, btn_len);
    
    grid.attach(&button_7, 0, 1, btn_len, btn_len);
    grid.attach(&button_8, 1, 1, btn_len, btn_len);
    grid.attach(&button_9, 2, 1, btn_len, btn_len);
    grid.attach(&button_mutiplier, 3, 1, btn_len, btn_len);
    
    grid.attach(&button_4, 0, 2, btn_len, btn_len);
    grid.attach(&button_5, 1, 2, btn_len, btn_len);
    grid.attach(&button_6, 2, 2, btn_len, btn_len);
    grid.attach(&button_minus, 3, 2, btn_len, btn_len);

    grid.attach(&button_1, 0, 3, btn_len, btn_len);
    grid.attach(&button_2, 1, 3, btn_len, btn_len);
    grid.attach(&button_3, 2, 3, btn_len, btn_len);
    grid.attach(&button_plus, 3, 3, btn_len, btn_len);
    
    grid.attach(&button_0, 0, 4, btn_len * 2, btn_len);
    grid.attach(&button_point, 2, 4, btn_len, btn_len);
    grid.attach(&button_equal, 3, 4, btn_len, btn_len);

    window.show();
}