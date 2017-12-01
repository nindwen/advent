pub mod summate;

#[cfg(test)]
mod day_1_part_1 {
    use summate;
    #[test]
    fn sum_1122() {
        assert_eq!(summate::part_one_sum("1122"), 3);
    }
    #[test]
    fn sum_1111() {
        assert_eq!(summate::part_one_sum("1111"), 4);
    }
    #[test]
    fn sum_1234() {
        assert_eq!(summate::part_one_sum("1234"), 0);
    }
    #[test]
    fn sum_91212129() {
        assert_eq!(summate::part_one_sum("91212129"), 9);
    }
    #[test]
    #[should_panic(expected = "Input is expected to be digits")]
    fn incorrect() {
        summate::part_one_sum("1234e");
    }
}

#[cfg(test)]
mod day_1_part_2 {
    use summate;
    #[test]
    fn sum_1212() {
        assert_eq!(summate::part_two_sum("1212"), 6);
    }
    #[test]
    fn sum_1221() {
        assert_eq!(summate::part_two_sum("1221"), 0);
    }
    #[test]
    fn sum_123425() {
        assert_eq!(summate::part_two_sum("123425"), 4);
    }
    #[test]
    fn sum_123123() {
        assert_eq!(summate::part_two_sum("123123"), 12);
    }
    #[test]
    fn sum_12131415() {
        assert_eq!(summate::part_two_sum("12131415"), 4);
    }
}
