extern crate failure;
pub mod checksum;
#[cfg(test)]
mod tests {
    use checksum;
    #[test]
    fn part1() {
        let result = checksum::checksum("test_data.txt").unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn part2() {
        let result = checksum::checksum_part2("test_data2.txt").unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    #[should_panic]
    fn null() {
        let _ = checksum::checksum_part2("test_data_null.txt").unwrap();
    }
}
