
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Dialog, Builder, ComboBox};

fn main() {
    if gtk::init().is_err() {
        println!("Couldn't initialize GTK!'");
        return;
    }

    let type_dialog_glade = include_str!("type-dialog.glade");
    let builder = Builder::new_from_string(type_dialog_glade);

    let dialog: Dialog = builder.get_object("type_dialog").expect("Couldn't get type_dialog");
    let type_list: ComboBox = builder.get_object("type_list").expect("Couldn't get type_list");

    // Set default to 'Screenshot'
    type_list.set_active(0);
    dialog.show_all();
    gtk::main();
}


