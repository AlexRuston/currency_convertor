use std::io;

const DOLLARS_TO_POUNDS: f64 = 0.75;
const POUNDS_TO_DOLLARS: f64 = 1.32;

fn main() {

    let choice;
    let sign;
    let converted_sign;

    loop {

        println!("Choose from the following options:\n1: Pounds to dollars\n2: Dollars to pounds");
        
        let mut input_text = String::new();
        
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read input");

        let num = input_text.trim().parse::<u64>().expect("That's not a number");

        // validate num
        match num {

            1 => {
                choice = "pounds";
                sign = "£";
                converted_sign = "$";
                break;
            },

            2 => {
                choice = "dollars";
                sign = "$";
                converted_sign = "£";
                break;
            },
            
            _ => {
                println!("Please choose an option from the list\n");
            }
        };
    }

    println!("Enter amount to convert:");
    
    let mut amount_text = String::new();
    
    io::stdin()
        .read_line(&mut amount_text)
        .expect("Failed to read input");

    let amount = amount_text
        .trim()
        .parse::<f64>()
        .expect("That's not a number");

    println!("{}{} is {}{:.2}", sign, amount, converted_sign, convert(amount, choice));
}

fn convert(amount: f64, choice: &str) -> f64 {
    match choice {
        "pounds" => amount * POUNDS_TO_DOLLARS,
        "dollars" => amount * DOLLARS_TO_POUNDS,
        _ => amount
    }
}