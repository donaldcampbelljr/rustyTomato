use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub fn build_ascii_numerals(filepath: &str) -> Vec<Vec<String>> {

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

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

    let mut new_numeral = Vec::new();
    // Make double digits.

    for number in 0..60 {
        let mut temp_numeral:Vec<String> = Vec::new();
        if number < 9 {
            for i in vec![0, 6, 12, 18, 24] {
                let string_slice1: &str = &numerals[0][i..(i + 6)];
                let string_slice2: &str = &numerals[number][i..(i + 6)];
                let concatenated: String = String::from(string_slice1) + "   " + string_slice2;
                temp_numeral.push(concatenated);
            }
            temp_numeral.join("\n");

        }
        else {

            let first:usize = (number / 10) as usize;
            let second:usize = (number%10) as usize;

            for i in vec![0, 6, 12, 18, 24] {
                let string_slice1: &str = &numerals[first][i..(i + 6)];
                let string_slice2: &str = &numerals[second][i..(i + 6)];
                let concatenated: String = String::from(string_slice1) + "   " + string_slice2;
                temp_numeral.push(concatenated);
            }

            temp_numeral.join("\n");



        }
        new_numeral.push(temp_numeral);
    }


    // Return vector containing string slices that represent the numerals.
    let numerals = new_numeral;
    numerals
}
