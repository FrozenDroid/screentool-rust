
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Dialog, DialogFlags, Label, ListBox, ListBoxRow, CheckButton};

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

    //dialog.get_style_context()
    //    .unwrap()
    //    .set_property("use-header-bar", &(1 as i32))
    //    .expect("Error while setting spacing");

    // dialog.set_property_margin(10);

    
    let content_area = Dialog::get_content_area(&dialog);

    content_area.set_property_margin(10);

    let label = Label::new(Some("Test"));
    let label2 = Label::new(Some("Booooop"));

    let list_box = ListBox::new();
    let list_box_row = ListBox::new();

    let color = gdk::RGBA { alpha: 100.0, blue: 255.0, green: 0.0, red: 0.0};

    let check_button = CheckButton::new_with_label("boooop");
    println!("{:?}", check_button.get_child().unwrap().downcast::<gtk::Label>().unwrap().set_markup("<span foreground=\"black\">boop!</span>"));
    //check_button.override_color(gtk::StateFlags::NORMAL, &color);
    list_box_row.add(&check_button);
    list_box.add(&list_box_row);

    content_area.add(&label);
    content_area.add(&list_box);
    // window.set_title("Test");
    // window.set_default_size(350, 70);

    //window.add(&dialog);

    dialog.show_all();

    gtk::main();
}

