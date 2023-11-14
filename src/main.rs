
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
    TimeItem(u64, TimeUnits),
    History(usize),
}


fn main() {

    //timer 25 min
    //hist 5 days
    //break 5 min

    // Build method NOT derive method

    // positional arguments
    // let match_result = command!().arg(
    //     Arg::new("firstname")
    // ).arg(
    //     Arg::new("lastname")
    // )
    //     .get_matches();
    // println!("{:?}", match_result);

    let match_result = command!().about("This application is a simple CLI pomodoro timer.").arg(
        Arg::new("timer").short('t').default_value("timer")
    ).arg(
        Arg::new("time_number").short('n').default_value("5")
    ).arg(
        Arg::new("time_units").short('u').default_value("s")
    )
        .get_matches();

    //println!("{:?}", match_result);

    let mut user_time_input: TimeUnitOrBreak;

    // If using `cargo run` in the top level folder.
    let numerals_file_path = Path::new("./src/ascii_art/ascii_numbers.txt");
    let session_file_path = Path::new("./src/history/session_history.txt");
    // Must convert &Path to &str and unwrap the result of the .to_str() func
    let numerals: Vec<Vec<String>> = numerals::build_ascii_numerals(numerals_file_path.to_str().unwrap());

    let func = match_result.get_one::<String>("timer").expect("expecting string").to_string();
    let time_number:u64 = match_result.get_one::<u64>("time_number").expect("expecting u64").to_string().parse().unwrap();
    let time_units = match_result.get_one::<String>("time_units").expect("expecting string").to_string();

    user_time_input = get_time_input(time_number, time_units);


    // Clear screen each time
    //print!("{esc}c", esc = 27 as char);
    println!("Welcome to the rsTomato! 🍅 \n");

    let begin_time  = Utc::now();

    match func.as_str(){
        "timer" => TimeUnits::Minutes(),
        "break" => TimeUnits::Minutes(),
        "history" => TimeUnits::Minutes(),
        _ => TimeUnits::Minutes(),
    }

    match user_time_input {
        TimeUnitOrBreak::Str(s) if s == "quit\n" => {


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

        TimeUnitOrBreak::History(time_bucket) =>{
            println!("Some history {}", time_bucket);
            plot_history(time_bucket);
        }

        TimeUnitOrBreak::Str(s) => {

            println!("CONINUING LOOP");

    },
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

fn get_time_input(time_number: u64, time_units: String) -> TimeUnitOrBreak {

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





