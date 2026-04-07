use std::rc::Rc;

use crate::{
    db::JiraDatabase,
    utils::{IO, Navigator},
};

pub mod db;
pub mod ui;
pub mod utils;

fn main() {
    let db = Rc::new(JiraDatabase::new("../data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "
                    Error rendering page: {}\nPress any key to continue...",
                    error
                );
                IO::wait_for_key_press();
            }

            let user_input = IO::input(None, "Invalid input provided");

            match page.handle_input(user_input.trim()) {
                Err(e) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        e
                    );
                    IO::wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error handling processing user input: {}\nPress any key to continue...",
                                error
                            );
                            IO::wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
