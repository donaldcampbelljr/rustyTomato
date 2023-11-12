use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use lowcharts::plot;
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, NaiveDateTime};



pub fn write_session_history(filepath: &str, begin:DateTime<Utc>,end:DateTime<Utc>) -> (){
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

pub fn plot_history(){

    //const FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S.%f UTC%z";
    // READ TEXT FILE
    let filepath = Path::new("./src/history/session_history.txt");
    let file = File::open(filepath).expect("Failed to read file; does it exist?");
    let mut reader = BufReader::new(file);

    let mut all_time_points: Vec<DateTime<FixedOffset>> = Vec::new();
    const OFFSET_STRING: &str = "+02:00";
    let offset: FixedOffset = OFFSET_STRING.parse().expect("Failed to parse offset");

    for line in reader.lines(){

        let line = line.expect("Failed to read line");
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        // "2023-11-10 09:22:02.785412 UTC  PT2.006392S"
        // "%Y-%m-%d %H:%M:%S.%f UTC%z   ????? "

        let date = NaiveDate::parse_from_str(line_parts[0], "%Y-%m-%d")
            .expect("Could not parse naive date.");

        let time = NaiveTime::parse_from_str(line_parts[1], "%H:%M:%S.%f")
            .expect("Could not parse naive time.");

        // This may not be the right offset...
        //let tz_offset = FixedOffset::east_opt(1 * 3600);
        let datetime = NaiveDateTime::new(date,time);
        let fixed_offset_datetime: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(datetime, offset);
        
        all_time_points.push(fixed_offset_datetime);

    }

    println!("\n\n TIME HISTOGRAM");

    let histogram = plot::TimeHistogram::new(7, &all_time_points);
    print!("{}", histogram);



}