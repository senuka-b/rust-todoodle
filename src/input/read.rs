use std::io::{self, Write};

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

