use gtk4 as gtk;
use gtk::prelude::*;
use glib::clone;
use gtk::glib;

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
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(0)
        .column_spacing(0)
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
    let button_clear = gtk::Button::with_label("AC");
    let button_sign = gtk::Button::with_label("+/-");
    let button_percent = gtk::Button::with_label("%");
    let button_divider = gtk::Button::with_label("÷");
    let button_mutiplier = gtk::Button::with_label("x");
    let button_minus = gtk::Button::with_label("-");
    let button_plus = gtk::Button::with_label("+");
    let button_equal = gtk::Button::with_label("=");
    let button_point = gtk::Button::with_label(".");

    let text_view = gtk::Entry::builder()
        .build();

    grid.attach(&text_view, 0, 0, 5, 1);

    grid.attach(&button_clear, 0, 1, btn_len, btn_len);
    grid.attach(&button_sign, 1, 1, btn_len, btn_len);
    grid.attach(&button_percent, 2, 1, btn_len, btn_len);
    grid.attach(&button_divider, 3, 1, btn_len, btn_len);
    
    grid.attach(&button_7, 0, 2, btn_len, btn_len);
    grid.attach(&button_8, 1, 2, btn_len, btn_len);
    grid.attach(&button_9, 2, 2, btn_len, btn_len);
    grid.attach(&button_mutiplier, 3, 2, btn_len, btn_len);
    
    grid.attach(&button_4, 0, 3, btn_len, btn_len);
    grid.attach(&button_5, 1, 3, btn_len, btn_len);
    grid.attach(&button_6, 2, 3, btn_len, btn_len);
    grid.attach(&button_minus, 3, 3, btn_len, btn_len);

    grid.attach(&button_1, 0, 4, btn_len, btn_len);
    grid.attach(&button_2, 1, 4, btn_len, btn_len);
    grid.attach(&button_3, 2, 4, btn_len, btn_len);
    grid.attach(&button_plus, 3, 4, btn_len, btn_len);
    
    grid.attach(&button_0, 0, 5, btn_len * 2, btn_len);
    grid.attach(&button_point, 2, 5, btn_len, btn_len);
    grid.attach(&button_equal, 3, 5, btn_len, btn_len);

    window.show();

    button_0.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "0");
        text_view.set_text(text.as_str());
    }));

    button_1.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "1");
        text_view.set_text(text.as_str());
    }));

    button_2.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "2");
        text_view.set_text(text.as_str());
    }));

    button_3.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "3");
        text_view.set_text(text.as_str());
    }));

    button_4.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "4");
        text_view.set_text(text.as_str());
    }));

    button_5.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "5");
        text_view.set_text(text.as_str());
    }));

    button_6.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "6");
        text_view.set_text(text.as_str());
    }));

    button_7.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "7");
        text_view.set_text(text.as_str());
    }));

    button_8.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "8");
        text_view.set_text(text.as_str());
    }));

    button_9.connect_clicked(clone!(@weak text_view => move |_btn| {
        let text = format!("{}{}", text_view.text().as_str(), "9");
        text_view.set_text(text.as_str());
    }));

    button_clear.connect_clicked(clone!(@weak text_view => move |_btn| {
        text_view.set_text("");
    }));
    /*

    let button_sign = gtk::Button::with_label("+/-");
    let button_percent = gtk::Button::with_label("%");
    let button_divider = gtk::Button::with_label("÷");
    let button_mutiplier = gtk::Button::with_label("x");
    let button_minus = gtk::Button::with_label("-");
    let button_plus = gtk::Button::with_label("+");
    let button_equal = gtk::Button::with_label("=");
    let button_point = gtk::Button::with_label(".");
}
*/
    button_sign.connect_clicked(clone!(@weak text_view => move |_btn| {
        let ch = text_view.text().as_str().chars().nth(0).unwrap();
        let str = String::from(text_view.text());
        if ch == '-' {
            let mut chars = str.chars();
            chars.next();
            text_view.set_text(chars.as_str());
        } else {
            let text = format!("{}{}", "-", text_view.text().as_str());
            text_view.set_text(text.as_str());
        }
    }));

    button_percent.connect_clicked(clone! (@weak text_view => move |_btn| {
        let str = String::from(text_view.text());
        let mut value = str.parse::<f64>().unwrap();
        value *= 0.01;
        text_view.set_text(value.to_string().as_str());
    }));
}