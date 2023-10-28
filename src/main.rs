
use std::io::{self,Write};

#[derive(Debug)]
pub enum TimeUnits{
    Minutes(),
    Seconds(),
}


fn main() {

    let mut user_time_input: (u64, TimeUnits);

    loop {
        println!("Welcome to the rusTomato! Input the desired timer length and press enter!\n\n");

        user_time_input = get_time_input();
        let calculated_time = create_time(user_time_input.0, user_time_input.1);

        // if matches!(command, rlib::Command::Quit) {
        //     break;
        //
        // }
        break
    }
    println!("Hello, world!");
    //println!("{:?}", calculated_time);
}



fn create_time(time_numbers: u64, time_tuple: TimeUnits) -> u64 {

    // take the user input and conver to mins and seconds
    println!("{}", time_numbers);
    if matches!(time_tuple, TimeUnits::Minutes()) {
        println!("time_units is the Minutes variant.");
    } else {
        println!("time_units is some other variant.");
    }

    600

}

fn get_time_input() -> (u64, TimeUnits) {

    // Prompting the User

    println!("");
    println!("> ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read input");

    println!("");

    let lc_input_str = input_str.to_lowercase(); // makes lower case
    let mut split_input_iter = lc_input_str.trim().split_whitespace(); // creates an iterable

    let time_number= split_input_iter.next().unwrap_or_default();// takes the 'next' str in the iterable (this is the first one)
    let time_number: u64 = time_number.parse().unwrap();
    // *time_number as u64;
    let time_unit = split_input_iter.next().unwrap_or_default().to_string(); // takes the 'next' str in the iterable (this is the second one)

     let time_units = match time_unit.as_str(){
        "min" => TimeUnits::Minutes(),
        "minutes" => TimeUnits::Minutes(),
        "m" => TimeUnits::Minutes(),
        "seconds" => TimeUnits::Seconds(),
        "sec" => TimeUnits::Seconds(),
        "s" => TimeUnits::Seconds(),
        _ => TimeUnits::Minutes(), // Basically just assume minutes
    };

    (time_number, time_units)

}

fn pretty_display() {

    // create a nice display of time


}
