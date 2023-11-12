
use std::io::{self, BufWriter, Write};
use std::{thread, time, path::Path};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use chrono::prelude::*;
use lowcharts::plot;
use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseError, Utc};


#[derive(Debug)]
pub enum TimeUnits{
    Minutes(),
    Seconds(),
}

enum TimeUnitOrBreak {
    Str(String),
    TimeItem(u64, TimeUnits),
}


fn main() {

    // Testing plotting
    print!("PLOT HISTORY TEST");
    plot_history();

    let mut user_time_input: TimeUnitOrBreak;

    // If using `cargo run` in the top level folder.
    let file_path = Path::new("./src/ascii_art/ascii_numbers.txt");
    let session_file_path = Path::new("./src/history/session_history.txt");
    //let numerals: Vec<Vec<String>> = build_ascii_numerals("/home/drc/GITHUB/rustyTomato/src/ascii_art/ascii_numbers.txt");
    // Must convert &Path to &str and unwrap the result of the .to_str() func
    let numerals: Vec<Vec<String>> = build_ascii_numerals(file_path.to_str().unwrap());

    loop {
        // Clear screen each time
        //print!("{esc}c", esc = 27 as char);
        println!("Welcome to the rsTomato! 🍅 \nInput the desired timer length and press enter!\n");

        user_time_input = get_time_input();

        let begin_time  = Utc::now();


        // if user_time_input = TimeUnitOrBreak::Str("quit".as_str()) { $
        //     break; $ $
        // }
        match user_time_input {
            TimeUnitOrBreak::Str(s) => {break;},
            TimeUnitOrBreak::TimeItem(time, units) => {

                let calculated_time = create_time(time, units);

                timer(calculated_time, numerals);

            },
        }

        // let calculated_time = create_time(user_time_input.0, user_time_input.1);
        //
        // timer(calculated_time, numerals);
        let end_time  = Utc::now();

        write_session_history(session_file_path.to_str().unwrap(), begin_time,end_time);

        break;
    }
    println!("Ending Program");
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

    match lc_input_str.as_str(){
        "quit\n" => TimeUnitOrBreak::Str(lc_input_str),
        _ => {
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

            TimeUnitOrBreak::TimeItem(time_number, time_units)

        }

    }



}

fn timer(time_seconds: u64, numerals: Vec<Vec<String>>){
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

fn pretty_display(min:u64, sec:u64, numerals: &Vec<Vec<String>>) {
    let min = min as usize;
    let sec = sec as usize;

    // Clear screen each time
    print!("{esc}c", esc = 27 as char);

    // create a nice display of time
    println!("{}   {}\n", min, sec);
    //println!("{}   {}", numerals.get(min).unwrap(), numerals.get(sec).unwrap())
    //println!("{}     {}", numerals.get(0).unwrap(), numerals.get(1).unwrap())
    // Minutes
    for j in 0..5 {
        println!("{}", numerals[min][j]);
    }
    // Seconds
    for j in 0..5 {
        println!("{}", numerals[sec][j]);
    }


}


fn write_session_history(filepath: &str, begin:DateTime<Utc>,end:DateTime<Utc>) -> (){
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(filepath)
        .expect("cannot open file");

    let begin_string = begin.to_string();
    let duration_string = end.signed_duration_since(begin).to_string();

    // concatenate our desired output string
    let final_string = begin_string + "  " + duration_string.as_str() + " \n";

    println!("Writing Session History...");
    println!("{}", final_string);
    // Write to a file
    data_file
        .write(final_string.as_bytes())
        .expect("write failed");

}

fn build_ascii_numerals(filepath: &str) -> Vec<Vec<String>> {

    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    // numerals will be final vector returned
    let mut temp_batch = Vec::new();
    let mut numerals:Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        temp_batch.push(line);

        if temp_batch.len() == 5 {
            let concatenated_lines: String = temp_batch.join("");

            // Had to push a String NOT a &str!
            // Original code has hard coded &str and that worked fine.
            numerals.push(concatenated_lines);

            temp_batch.clear();
        }
    }

    // println!("The length of the vector is {}", numerals.len());
    // for k in 0..10 {
    //     println!("{}", numerals.get(k).unwrap());
    //     println!("\n");
    // }


    let mut new_numeral = Vec::new();
    // Make double digits.

    for number in 0..60 {
        // println!("{}",number);
        let mut temp_numeral:Vec<String> = Vec::new();
        if number < 9 {
            for i in vec![0, 6, 12, 18, 24] {
                let string_slice1: &str = &numerals[0][i..(i + 6)];
                let string_slice2: &str = &numerals[number][i..(i + 6)];
                let concatenated: String = String::from(string_slice1) + "   " + string_slice2;
                temp_numeral.push(concatenated);
            }
            temp_numeral.join("\n");

            // for k in 0..5 {
            //     println!("{}", temp_numeral.get(k).unwrap());
            // }
        }
        else {

            // // This way is difficult road to travel, just use Math.
            // // need to take the number, deconstruct it into digits and then use that digit for each respective slices.
            // let number_string = number.to_string();//String::from(number);
            // // Get an iterator over the characters in the number.
            // //let mut chars = number_string.chars();
            // // Split the number at the first character.
            // let first = &number_string[0];
            // let second = &number_string[1];
            // let first_number:u32 = first.to_digit(10).unwrap();
            // println!("FIRST NUMBER: {}", first_number);
            // let second_number:u32 = second.to_digit(10).unwrap();
            // println!("Second NUMBER: {}", second_number);

            let first:usize = (number / 10) as usize;
            // println!("FIRST NUMBER: {}", first);
            let second:usize = (number%10) as usize;
            // println!("Second NUMBER: {}", second);

            for i in vec![0, 6, 12, 18, 24] {
                let string_slice1: &str = &numerals[first][i..(i + 6)];
                let string_slice2: &str = &numerals[second][i..(i + 6)];
                let concatenated: String = String::from(string_slice1) + "   " + string_slice2;
                temp_numeral.push(concatenated);
            }

            temp_numeral.join("\n");

            // for k in 0..5 {
            //     println!("{}", temp_numeral.get(k).unwrap());
            // }


        }
        new_numeral.push(temp_numeral);
    }


    // println!("{:?}", new_numeral);
    // for k in 0..5 {
    //     for j in 0..5 {
    //     println!("{}", new_numeral[k][j]);
    // }
    // }

    //println!("DONE");
    // Return vector containing string slices that represent the numerals.
    let numerals = new_numeral;
    numerals
}

fn plot_history(){

    const FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S.%f UTC%z";
    // READ TEXT FILE
    let filepath = Path::new("./src/history/test_history.txt");
    let file = File::open(filepath).expect("Failed to read file; does it exist?");
    let mut reader = BufReader::new(file);

    //let line = "2023-11-6";
    let line2 = "2023-11-10 09:22:02.785412 UTC";
    let naive_date = NaiveDate::parse_from_str(line2, "%Y-%m-%d %H:%M:%S.%f UTC").expect("Could not parse naive date.");
    //let begin_time  = Utc::now().to_string();
    //let datetime = DateTime::parse_from_str(&begin_time, FORMAT_STRING).expect("Failed to parse datetime");
    let day_as_int = naive_date.day().to_string().parse::<u16>().unwrap();
    println!("TESTING DATE {} \n", naive_date.day());
    println!("day as int {} \n", day_as_int);

    let sv1:Vec<&str> = line2.split_whitespace().collect();
    for x in sv1.iter() {
        println!("{}", x);
    }

    let line3 = "2023-11-6 09:22:02.785412 UTC 2023-11-7 09:22:02.785412 UTC";
    //let naive_date3 = NaiveDate::parse_from_str(line3, "%Y-%m-%d %H:%M:%S.%f UTC %Y-%m-%d %H:%M:%S.%f UTC").expect("Could not parse naive date.");

    let parts: Vec<&str> = line3.split(' ').collect();
    let datetime1 = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d")
        .expect("Failed to parse datetime");
    //let datetime2 = NaiveDate::parse_from_str(parts[1], "%H:%M:%S.%f")
    //.expect("Failed to parse datetime");

    println!("\n datetime1   {}", datetime1); //2023-11-6
    println!("\n parts[1]  {}", parts[1]); //09:22:02.785412
    println!("\n parts[2]  {}", parts[2]); //UTC
    println!("\n parts[3]  {}", parts[3]); // 2023-11-6
    println!("\n parts[4]  {}", parts[4]); //09:22:02.785412
    println!("\n parts[5]  {}", parts[5]); //UTC


    let line4 = "2023-11-10 09:22:02.785412 UTC  PT2.006392S";
    let parts_line4: Vec<&str> = line4.split_whitespace().collect();

    println!("\n\n");
    println!("{} {} {} {}", parts_line4[0], parts_line4[1], parts_line4[2], parts_line4[3]);


        // for line in reader.lines(){
        //     let line = line.expect("Failed to read line");
        //     let offset: FixedOffset = "+02:00".parse().expect("Failed to parse offset");
        //     let datetime = DateTime::parse_from_str(&line, FORMAT_STRING).expect("Failed to parse datetime");
        //     println!("{} {} {}",line, offset, datetime);
        // }
    // let mut tokens;
    // for line in reader.lines() {
    //     // Split the line into tokens
    //     tokens = line.unwrap().split(' ');

    //     // Take the first token, which is the date
    //     let date = tokens.next().unwrap();
    //     //
    //     // // Print the date
    //     //println!("{:?}", date);
    // }


    let vec = &[-1.0, -1.1, 2.0, 2.0, 2.1, -0.9, 11.0, 11.2, 1.9, 1.99];
// Plot a histogram of the above vector, with 4 buckets and a precision
// chosen by library
    let options = plot::HistogramOptions { intervals: 4, ..Default::default() };
    let histogram = plot::Histogram::new(vec, options);
    print!("{}", histogram);

}