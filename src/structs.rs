
use crate::input;
use chrono;

pub struct Todo {
    pub title: String,
    description: String,
    date_created: i64,
    date_due: i64,
    completed: bool,
} 

impl Todo {
    pub fn new() -> Todo {
        let title = input::read::read_string("Enter title", true);
        let description = input::read::read_string("Enter description",  false);

        let date_created = chrono::Utc::now().timestamp();

        let date_due = input::parse::date();

        let completed = false;

        Todo {
            title,
            description,
            date_created,
            date_due,
            completed
        }
    }
}