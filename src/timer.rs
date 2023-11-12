use std::{thread, time};

pub fn timer(time_seconds: u64, numerals: Vec<Vec<String>>){
    // takes in time and detracts it after pausing for one second

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

pub fn pretty_display(min:u64, sec:u64, numerals: &Vec<Vec<String>>) {
    let min = min as usize;
    let sec = sec as usize;

    // Clear screen each time
    print!("{esc}c", esc = 27 as char);

    // create a nice display of time
    println!("{}   {}\n", min, sec);

    // Minutes
    for j in 0..5 {
        println!("{}", numerals[min][j]);
    }
    // Seconds
    for j in 0..5 {
        println!("{}", numerals[sec][j]);
    }


}