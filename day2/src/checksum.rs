use std::fs::File;
use std::io::prelude::*;
use failure::Error;

fn read_file(filename: &str) -> Result<Vec<Vec<i32>>, Error> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    let mut sheet = Vec::new();
    f.read_to_string(&mut contents)?;

    for line in contents.lines() {
        sheet.push(Vec::new());
        for column in line.split("\t") {
            let number: i32 = column.parse()?;
            sheet.last_mut().unwrap().push(number);
        }
    }

    Ok(sheet)
}

pub fn checksum(filename: &str) -> Result<i32, Error> {
    let sheet = read_file(filename)?;
    Ok(sheet.iter().fold(0, |acc, row| {
        let min = row.iter().min().unwrap_or(&0);
        let max = row.iter().max().unwrap_or(&0);
        acc + max - min
    }))
}

pub fn checksum_part2(filename: &str) -> Result<i32, Error> {
    let sheet = read_file(filename)?;
    Ok(sheet.iter().fold(0, |acc, row| {
        let mut division = 0;
        for numerator in row {
            for denominator in row {
                if denominator != &0 
                    && numerator != denominator 
                    && numerator % denominator == 0 {
                    division = numerator / denominator;
                }
            }
        }
        return acc + division;
    }))
}
