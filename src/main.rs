extern crate systray;
use std::{thread, time};

fn main() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Unable to create window")
    }

    app.set_icon_from_file(&".\\heart.ico".to_string()).unwrap();
    app.add_menu_item(&"Exit".to_string(), |w| {
        w.quit();
    }).unwrap();
    app.wait_for_message();
}
