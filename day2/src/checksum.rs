
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
    let mut sum = 0;
    for row in sheet {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        sum += max - min;
    }
    Ok(sum)
}

pub fn checksum_part2(filename: &str) -> Result<i32, Error> {
    let sheet = read_file(filename)?;
    let mut sum = 0;
    for row in sheet {
        for divisor in &row {
            for divident in &row {
                if divisor != divident && divisor % divident == 0 {
                    let division = divisor.checked_div(*divident)
                        .expect("Division by zero");
                    sum += division;
                }
            }
        }
    }
    Ok(sum)
}
