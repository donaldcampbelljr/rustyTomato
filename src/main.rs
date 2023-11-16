use std::{io, path::Path, path::PathBuf};
use std::io::Write;
use chrono::{Utc};
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
    TimeItem(usize, TimeUnits),
}

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()>  {

    let mut match_result = command!().about("This application is a simple CLI pomodoro timer.").arg(
        Arg::new("timer").short('t').default_value("timer")
    ).arg(
        Arg::new("time_number").short('n').default_value("5").value_parser(clap::value_parser!(usize))
    ).arg(
        Arg::new("time_units").short('u').default_value("s")
    )
        .get_matches();

    let mut user_time_input: TimeUnitOrBreak;

    // Must obtain crate path such that build can find the resources.
    let path_to_crate= env!("CARGO_MANIFEST_DIR");
    //let numerals_file_path = Path::new("./src/ascii_art/ascii_numbers.txt");
    let numerals_file_path = PathBuf::from(path_to_crate).join(Path::new("./src/ascii_art/ascii_numbers.txt"));
    //let session_file_path = Path::new("./src/history/session_history.txt");
    let session_file_path = PathBuf::from(path_to_crate).join(Path::new("./src/history/session_history.txt"));
    // Must convert &Path to &str and unwrap the result of the .to_str() func
    let numerals: Vec<Vec<String>> = numerals::build_ascii_numerals(numerals_file_path.to_str().unwrap());

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Welcome to the rsPomodoro! 🍅 \n")
                    .red()
                    .on_white(),
                area,
            );
        })?;

        let func = match_result.get_one::<String>("timer").expect("expecting string").to_string();
        let time_number: usize = match_result.get_one::<usize>("time_number").expect("expecting u64").to_string().parse().unwrap();
        let time_units = match_result.get_one::<String>("time_units").expect("expecting string").to_string();

        user_time_input = get_time_input(time_number, time_units);

        //println!("Welcome to the rsPomodoro! 🍅 \n");

        let begin_time = Utc::now();

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
                        plot_history(time)
                    },
                    "q" => {
                        // stdout().execute(LeaveAlternateScreen)?;
                        // disable_raw_mode()?;
                        // Ok(())

                    },
                    _ => {
                        println!("Input function does not exist...Ending Program...")
                    },
                }
            }
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        println!(">  ");
        io::stdout().flush().unwrap();

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read input");

        println!("");

        let lc_input_str = input_str.to_lowercase(); // makes lower case
        // match lc_input_str.as_str() {
        //     "quit\n" => break,
        //     _ => continue,
        // }
        let input: Vec<_> = lc_input_str.split_whitespace()
            .map(|s| s.to_string()).collect();

        match_result = command!().about("This application is a simple CLI pomodoro timer.").arg(
            Arg::new("timer").short('t').default_value("timer")
        ).arg(
            Arg::new("time_number").short('n').default_value("5").value_parser(clap::value_parser!(usize))
        ).arg(
            Arg::new("time_units").short('u').default_value("s")
        )
            .get_matches_from(input);


    }
    println!("Ending Program");
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())

}



fn create_time(time_numbers: usize, time_tuple: TimeUnits) -> usize {

    let time_in_seconds:usize;

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





