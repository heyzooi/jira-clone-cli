use std::rc::Rc;

mod models;
mod db;
mod ui;
mod io_utils;

use anyhow::Error;
use ui::EpicDetail;
use ui::HomePage;
use db::JiraDatabase;
use db::JSONFileDatabase;
use ui::Page;
use ui::StoryDetail;

fn main() -> Result<(), Error> {
    let file_path = "data/db.json";

    let home_page = HomePage {
        db: Rc::new(JiraDatabase {
            database: Box::new(JSONFileDatabase {
                file_path: file_path.to_string(),
            })
        })
    };
    home_page.draw_page()?;

    let epic_detail_page = EpicDetail {
        epic_id: 1,
        db: Rc::new(JiraDatabase {
            database: Box::new(JSONFileDatabase {
                file_path: file_path.to_string(),
            })
        })
    };
    epic_detail_page.draw_page()?;

    let story_detail_page = StoryDetail {
        epic_id: 1,
        story_id: 2,
        db: Rc::new(JiraDatabase {
            database: Box::new(JSONFileDatabase {
                file_path: file_path.to_string(),
            })
        })
    };
    story_detail_page.draw_page()?;

    Ok(())
}
