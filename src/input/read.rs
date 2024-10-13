use std::io::{self, Read, Write};

pub fn read_choice(choices : Vec<u32>) -> u32 {
    let mut input = String::new();

    print!("\tEnter your option (0 to exit): "); // unlike println! prinl doesn't flush bc no newline so print gets skipped
    io::stdout().flush().unwrap(); // Manually flush the stdout buffer

    match io::stdin().read_line(&mut input) {
        Err(_error) => println!("Reading input failed"),
        _ => (),
    }

    println!();

    match input.trim().parse() {
        Ok(num) => {
            if num == 0 {
                return num; // Exit program
            }

            let is_valid = validate_choices(&choices, &num);

            if is_valid {
                return num;
            }

            println!("Invalid choice! Please choose a number in the range {:?}\n", choices);

            read_choice(choices)

        },
        Err(_) => {
            println!("Please enter a valid number!\n");
            read_choice(choices)
        }
    }
}

fn validate_choices(choices : &Vec<u32>, num : &u32 ) -> bool {
    // choices.contains(num)

    for i in choices {
        if i == num {
            return true;
        }
    }

    return false;
}


pub fn read_string(text: &str, required: bool) -> String {
    let mut input = String::new();

    print!("\n\t{text} : "); 
    io::stdout().flush().unwrap(); 

    match io::stdin().read_line(&mut input) {
        Err(_error) => println!("\tReading input failed"),
        _ => (),
    }

    if required && input.trim().is_empty() {
        println!("\nThis is a required field that you cannot leave empty! Try again\n");

        return read_string(text, required);
    }

    input

}

pub fn read_long_string(text: &str, exit_char: &str) -> String {
    let mut input = String::new();

    print!("\n\t{text} ({exit_char} and hit enter to end) : ");
    io::stdout().flush().unwrap(); 


    loop {
        io::stdin().read_line(&mut input).expect("Unable to read input");

        if input.trim().ends_with(exit_char) {
            let _ = input.split_off(input.trim().len() - exit_char.len());
            break;
        }

    }


    input
}