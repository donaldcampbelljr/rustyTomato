
use std::io::{self, BufWriter, Write};
use std::{thread, time, path::Path};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use chrono::format::Fixed;
use chrono::{prelude::*, naive};
use lowcharts::plot;
use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseError, Utc};
use crate::history::plot_history;

use clap::{Arg, command};

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
    TimeItem(usize, TimeUnits),
    History(usize),
}


fn main() {

    let match_result = command!().about("This application is a simple CLI pomodoro timer.").arg(
        Arg::new("timer").short('t').default_value("timer")
    ).arg(
        Arg::new("time_number").short('n').default_value("5").value_parser(clap::value_parser!(usize))
    ).arg(
        Arg::new("time_units").short('u').default_value("s")
    )
        .get_matches();

    let mut user_time_input: TimeUnitOrBreak;

    // If using `cargo run` in the top level folder.
    let numerals_file_path = Path::new("./src/ascii_art/ascii_numbers.txt");
    let session_file_path = Path::new("./src/history/session_history.txt");
    // Must convert &Path to &str and unwrap the result of the .to_str() func
    let numerals: Vec<Vec<String>> = numerals::build_ascii_numerals(numerals_file_path.to_str().unwrap());

    let func = match_result.get_one::<String>("timer").expect("expecting string").to_string();
    let time_number:usize = match_result.get_one::<usize>("time_number").expect("expecting u64").to_string().parse().unwrap();
    let time_units = match_result.get_one::<String>("time_units").expect("expecting string").to_string();

    user_time_input = get_time_input(time_number, time_units);

    println!("Welcome to the rsTomato! 🍅 \n");

    let begin_time  = Utc::now();

    match user_time_input {
        TimeUnitOrBreak::TimeItem(time, units) => {
        match func.as_str() {
            "timer" => {
                let calculated_time = create_time(time, units);
                timer::timer(calculated_time, numerals.clone());
                let end_time = Utc::now();
                history::write_session_history(session_file_path.to_str().unwrap(), begin_time, end_time);
            },
            "break" => {
                // default to 5 minutes for a break
                let calculated_time = create_time(300, TimeUnits::Seconds());
                timer::timer(calculated_time, numerals.clone());
            },
            "history" => {

                // Just take the input time numbers for now and treat them as buckets...
                plot_history(5)
            },
            _ => {
                println!("Input function does not exist...Ending Program...")
            },
        }

        }
        _ => {
            println!("Time and/or Units not Valid...Ending Program...")
        }
    }

    println!("Ending Program");
}



fn create_time(time_numbers: usize, time_tuple: TimeUnits) -> usize {

    let mut time_in_seconds:usize = 1*10;

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

fn get_time_input(time_number: usize, time_units: String) -> TimeUnitOrBreak {

    // Note, had to use match statement and pass a String to TimeUnitorBreak not a &str
    // because the &str will no longer exist out of scope but the String will?

    let lc_input_str = time_units.to_lowercase(); // makes lower case
    //let mut split_input_iter: Vec<&str> = lc_input_str.trim().split_whitespace().collect();

    let time_units = match time_units.as_str(){
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





