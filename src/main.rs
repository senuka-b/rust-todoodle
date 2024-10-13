mod input;
mod constants;
mod enums;
mod structs;

use enums::HomeChoices;

fn main() {
    home_page();

    clear_console();

    println!("Goodbye!");
}

fn home_page() {
    clear_console();

    println!("\n\tWelcome to the home page of Rust-Todoodle!\n");

    println!("{}", constants::HOME_PAGE_STRING);

    let input: u32 = input::read::read_choice(vec![1,2,3]);

    let choice: Option<HomeChoices> = HomeChoices::get_option(input);

    match choice {
        Some(v) => {
            match v {
                HomeChoices::CreateTodo => create_todo(),

                _ => ()
            }
        },

        None => return
        
    }


    home_page();
}


fn create_todo() {
    clear_console();

    println!("{}", constants::CREATE_TODO);

    let todo: structs::Todo = structs::Todo::new();

    println!("\n\t A todo with the name \"{}\" has been created succesfully!", todo.title);

    let yn: bool = input::read::yn_choice("Do you want to create another Todo?");

    if yn {
        create_todo();
    }

}


fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}