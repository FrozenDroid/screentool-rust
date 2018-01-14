
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Dialog, DialogFlags, Label, ListBox, ListBoxRow, CheckButton, RadioButton};

fn main() {
    if gtk::init().is_err() {
        println!("Couldn't initialize GTK!'");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    let dialog = Dialog::new_with_buttons(
        Some("Test dialog"), 
        Some(&window), 
        DialogFlags::DESTROY_WITH_PARENT,
        &[(&"Hello", 1)]
    );

    let content_area = Dialog::get_content_area(&dialog);

    content_area.set_property_margin(10);

    let label = Label::new(Some("Test"));

    let list_box = ListBox::new();
    add_option_row("Boop", &list_box);
    add_option_row("Boop1", &list_box);
    add_option_row("Boop2", &list_box);

    content_area.add(&label);
    content_area.add(&list_box);
    dialog.show_all();

    gtk::main();
}

fn add_option_row(s: &str, list_box: &ListBox) {
    let list_box_row = ListBoxRow::new();
    let children = list_box.get_children();
    let radio_button: RadioButton;

    let last_child = children.last();
    if let Some(last_child) = last_child {
        radio_button = RadioButton::new_with_label_from_widget(
            &last_child.clone()
                .downcast::<ListBoxRow>()
                .unwrap()
                .get_children()
                .first()
                .unwrap()
                .clone()
                .downcast::<RadioButton>()
                .unwrap(), 
            ""
        );
    } else {
        radio_button = RadioButton::new_with_label("");
    }

    radio_button.get_child()
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap()
        .set_markup(format!("<span foreground=\"black\">{}</span>", s).as_str());

    list_box_row.add(&radio_button);
    list_box.add(&list_box_row);
}

