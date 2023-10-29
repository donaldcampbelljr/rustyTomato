
use std::io::{self,Write};
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
pub enum TimeUnits{
    Minutes(),
    Seconds(),
}


fn main() {

    let mut user_time_input: (u64, TimeUnits);

    let numerals: Vec<String> = build_ascii_numerals("/Users/drcwork/RustroverProjects/rsPomodoro/src/ascii_art/ascii_numbers.txt");

    loop {
        println!("Welcome to the rustyTomato! Input the desired timer length and press enter!\n");

        user_time_input = get_time_input();
        let calculated_time = create_time(user_time_input.0, user_time_input.1);

        timer(calculated_time, numerals);

        break;
    }
    println!("Hello, world!");
    //println!("{:?}", calculated_time);
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

fn timer(time_seconds: u64, numerals: Vec<String>){
    // takes in time and detracts it after pausing for one second

    //let numerals: Vec<&str> = numerals;
    let mut time: u64 = time_seconds;
    let ten_millis: time::Duration = time::Duration::from_millis(1000);
    loop {


        thread::sleep(ten_millis);
        time = time - 1;

        let min = time / 60;
        let seconds = time % 60;

        pretty_display(min, seconds, &numerals);

        if time < 1 {
            println!("\nTIME DONE\n");
            break;
        }


    }



}

fn pretty_display(min:u64, sec:u64, numerals: &Vec<String>) {
    let min = min;
    let sec = sec;
    // create a nice display of time
    println!("{}   {}", min, sec);
    println!("{}", numerals.get(0).unwrap())


}

fn build_ascii_numerals(filepath: &str) -> Vec<String> {

    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    // numerals will be final vector returned
    let mut temp_batch = Vec::new();
    let mut numerals:Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        temp_batch.push(line);

        if temp_batch.len() == 5 {
            let concatenated_lines: String = temp_batch.join("\n");

            // Had to push a String NOT a &str!
            // Original code has hard coded &str and that worked fine.
            numerals.push(concatenated_lines);

            temp_batch.clear();
        }
    }

    // Hard code for now, read from text file later!
    // let zero: &str =  " 0000\n00  00\n00  00\n00  00\n 0000";
    // let one: &str = "1111\n  11\n  11\n  11\n111111";
    // numerals.push(zero);
    // numerals.push(one);

    // Return vector containing string slices that represent the numerals.
    numerals

}
