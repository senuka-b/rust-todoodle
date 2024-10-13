
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
        let title: String = input::read::read_string("Enter title", true);

        let description: String = input::read::read_long_string("Enter description", "-1");

        let date_created: i64 = chrono::Utc::now().timestamp();

        let date_due: i64 = input::parse::date();

        let completed: bool = false;

        Todo {
            title,
            description,
            date_created,
            date_due,
            completed
        }
    }
}