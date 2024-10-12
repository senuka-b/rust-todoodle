use std::process::Command;

mod input;
mod constants;

fn main() {
    home_page();

    clear_console();

    println!("Goodbye!");
}

fn home_page() {
    clear_console();

    println!("\n\tWelcome to the home page of Rust-Todoodle!\n");

    println!("{}", constants::HOME_PAGE_STRING);

    let choice = input::read::read_choice(vec![1,2,3]);

    if choice == 0 {return;}

    home_page();
}

fn clear_console() {
    Command::new("cmd").arg("/c").arg("cls");
}