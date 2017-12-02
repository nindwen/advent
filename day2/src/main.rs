extern crate day2;
use day2::checksum;

fn main() {
    let result = checksum::checksum("input.txt").unwrap();
    let result_part2 = checksum::checksum_part2("input.txt").unwrap();
    println!("Checksum: {}", result);
    println!("Checksum 2: {}", result_part2);
}
