use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

use anyhow::Result;

fn main() -> Result<()> {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_owned()));
    let mut navigator = Navigator::new(db);
    
    // 1. get current page from navigator. If there is no current page exit the loop.
    while let Some(current_page) = navigator.get_current_page() {
        clearscreen::clear().unwrap();
        
        // 2. render page
        if let Err(error) = current_page.draw_page() {
            println!("Error rendering page: {error}\nPress any key to continue...");
            wait_for_key_press();
            continue;
        };

        // 3. get user input
        let user_input = get_user_input();

        // 4. pass input to page's input handler
        let action_result = current_page.handle_input(&user_input);
        if let Err(error) = action_result {
            println!("Error handling input: {error}\nPress any key to continue...");
            wait_for_key_press();
            continue;
        }
        let action = action_result?;

        // 5. if the page's input handler returns an action let the navigator process the action
        if let Some(action) = action {
            navigator.handle_action(action)?;
        }
    }

    Ok(())
}