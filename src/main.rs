
use std::io::{self, BufWriter, Write};
use std::{thread, time, path::Path};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use chrono::format::Fixed;
use chrono::{prelude::*, naive};
use lowcharts::plot;
use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseError, Utc};

mod history;
mod timer;
mod numerals;

#[derive(Debug)]
pub enum TimeUnits{
    Minutes(),
    Seconds(),
    Days(),
    Months(),
}

enum TimeUnitOrBreak {
    Str(String),
    TimeItem(u64, TimeUnits),
    History(u16),
}


fn main() {

    let mut user_time_input: TimeUnitOrBreak;

    // If using `cargo run` in the top level folder.
    let numerals_file_path = Path::new("./src/ascii_art/ascii_numbers.txt");
    let session_file_path = Path::new("./src/history/session_history.txt");
    // Must convert &Path to &str and unwrap the result of the .to_str() func
    let numerals: Vec<Vec<String>> = numerals::build_ascii_numerals(numerals_file_path.to_str().unwrap());

    loop {
        // Clear screen each time
        //print!("{esc}c", esc = 27 as char);
        println!("Welcome to the rsTomato! 🍅 \nInput the desired timer length and press enter!\n");

        user_time_input = get_time_input();

        let begin_time  = Utc::now();

        match user_time_input {
            TimeUnitOrBreak::Str(s) if s == "quit\n" => {
                
                break;
            
            },
            TimeUnitOrBreak::Str(s) if s == "break\n" => {

                // default to 5 minutes for a break
                let calculated_time = create_time(300, TimeUnits::Seconds());
                timer::timer(calculated_time, numerals.clone());
        
        },
            TimeUnitOrBreak::TimeItem(time, units) => {

                let calculated_time = create_time(time, units);

                timer::timer(calculated_time, numerals.clone());

                let end_time  = Utc::now();

                history::write_session_history(session_file_path.to_str().unwrap(), begin_time,end_time);

            },

            TimeUnitOrBreak::History(_) =>{
                println!("CONINUING LOOP");
                continue;
            }

            TimeUnitOrBreak::Str(s) => {

                println!("CONINUING LOOP");
                continue;
        
        },
        }

    }

    println!("Ending Program");
}



fn create_time(time_numbers: u64, time_tuple: TimeUnits) -> u64 {

    let mut time_in_seconds:u64 = 1*10;

    // take the user input and conver to mins and seconds
    println!("{} == time_numbers", time_numbers);

    if matches!(time_tuple, TimeUnits::Seconds()) {
        println!("time_units is the seconds variant.");
        time_in_seconds = time_numbers;
    } else {
        println!("time_units is some other variant, assuming minutes");
        time_in_seconds = time_numbers*60;
    }

    time_in_seconds

}

fn get_time_input() -> TimeUnitOrBreak {

    // Prompting the User

    println!("e.g. 25 min, 300 seconds");
    println!(">  ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read input");

    println!("");


    // Note, had to use match statement and pass a String to TimeUnitorBreak not a &str
    // because the &str will no longer exist out of scope but the String will?

    let lc_input_str = input_str.to_lowercase(); // makes lower case
    let mut split_input_iter: Vec<&str> = lc_input_str.trim().split_whitespace().collect();

    if split_input_iter.len() == 1{

        match split_input_iter[0]{
            "quit\n" => TimeUnitOrBreak::Str(String::from(split_input_iter[0])),
            "break\n" => TimeUnitOrBreak::Str(String::from(split_input_iter[0])),
            "hist\n" => TimeUnitOrBreak::Str(String::from(split_input_iter[0])),
            _ => TimeUnitOrBreak::Str(String::from("break"))
        }
    }

    else if split_input_iter.len() == 2 {

        let time_number: u64 = split_input_iter[0].parse().unwrap();
        let time_unit = split_input_iter[1].to_string(); // takes the 'next' str in the iterable (this is the second one)

        let time_units = match time_unit.as_str(){
            "min" => TimeUnits::Minutes(),
            "minutes" => TimeUnits::Minutes(),
            "m" => TimeUnits::Minutes(),
            "seconds" => TimeUnits::Seconds(),
            "sec" => TimeUnits::Seconds(),
            "s" => TimeUnits::Seconds(),
            _ => TimeUnits::Minutes(), // Basically just assume minutes
        };

        TimeUnitOrBreak::TimeItem(time_number, time_units)

    }

    else{
        TimeUnitOrBreak::Str(String::from("break"))
    }

}


