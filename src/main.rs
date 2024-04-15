use std::collections::HashMap;
use std::env;
use std::path::Path;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box};
use rusty_sheet_core::*;

fn main()
{
    let mut character_list: HashMap<Uuid, Character> = HashMap::new();
    let mut race_list: HashMap<Uuid, Race> = HashMap::new();
    let mut class_list: HashMap<Uuid, Class> = HashMap::new();
    let mut item_list: HashMap<Uuid, Item> = HashMap::new();

    let current_dir = get_current_working_dir();
    let mut path: &Path = Path::new::<String>(&current_dir);
    let binding = path.join("data");
    path = binding.as_path();
    load_file(path, &mut race_list, &mut class_list, &mut item_list, &mut character_list);

    let app = Application::builder()
    .application_id("com.amiroof.rustysheet")
    .build();

    app.connect_activate(build_main_menu);

    app.run();
}

fn build_main_menu(app: &Application)
{

    let character_creation = Button::builder()
    .label("Create a new character")
    .margin(12)
    .build();

    let view_characters = Button::builder()
    .label("View character list")
    .margin(12)
    .build();

    let menu = Box::new(gtk::Orientation::Vertical, 0);
    menu.add(&character_creation);
    menu.add(&view_characters);

    let character_creation = Box::new(gtk::Orientation::Vertical, 0);


    let window = ApplicationWindow::builder()
    .title("Rusty Sheet")
    .application(app)
    .child(&menu)
    .build();

    window.show_all();
    window.show();
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}