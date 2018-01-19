
extern crate gtk;
extern crate gdk;
extern crate clap;

use gtk::prelude::*;
use gtk::{Dialog, Builder, ComboBoxText, Button};
use clap::{App, Arg};
use std::process::exit;
use std::rc::Rc;
use std::cell::Cell;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn action_from_string(string: String) -> Action {
    match string.to_lowercase().as_str() {
        "screenshot" => Action::Screenshot,
        "record" | "recording" => Action::Record,
        _ => Action::Cancel
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Couldn't initialize GTK!");
        return;
    }

    let matches = App::new("screentool-rust").version("0.1")
                    .arg(Arg::with_name("action")
                        .short("a")
                        .long("action")
                        .help("Takes in either 'screenshot' or 'record'")
                        .takes_value(true))
                    .get_matches();

    let action = matches.value_of("action").map(|s| action_from_string(s.to_string())).unwrap_or_else(type_dialog);

    println!("{:?}", action);

    match action {
        Action::Screenshot => take_screenshot(),
        Action::Record => record_screen(),
        _ => exit(0) 
    }

}

#[derive(Clone, Copy, Debug)]
enum Action {
    Screenshot,
    Record,
    Cancel
}

fn type_dialog() -> Action {
    let type_dialog_glade = include_str!("type-dialog.glade");
    let builder = Builder::new_from_string(type_dialog_glade);

    let dialog: Dialog = builder.get_object("type_dialog").expect("Couldn't get type_dialog");
    let type_list: ComboBoxText = builder.get_object("type_combobox").expect("Couldn't get type_list");
    let continue_button: Button = builder.get_object("continue_button").expect("Couldn't get continue_button");
    let cancel_button: Button = builder.get_object("cancel_button").expect("Couldn't get cancel_button");

    let action = Rc::new(Cell::new(Action::Cancel));

    type_list.set_active(0);

    dialog.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    cancel_button.connect_clicked(move |_| {
        gtk::main_quit();
    });

    continue_button.connect_clicked(clone!(action, type_list => move |_| {
        println!("{:?}", type_list.get_active_text());
        if let Some(active_text) = type_list.get_active_text() {
            action.set(action_from_string(active_text));
        }
        gtk::main_quit();
    }));

    dialog.show_all();
    gtk::main();

    return action.get();
}

fn record_screen() {
}

fn take_screenshot() {
}

