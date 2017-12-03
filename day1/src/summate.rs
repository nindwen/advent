fn str_to_vec(input: &str) -> Vec<u32> {
    let mut vector = Vec::new();
    for char in input.chars() {
        vector.push(char.to_digit(10).expect("Input is expected to be digits"));
    }
    vector
}

fn summate(list: &[u32], offset: usize) -> u32 {
    let mut sum = 0;
    for (i, item) in list.iter().enumerate() {
        let index = (i + offset) % list.len();
        if *item == list[index] {
            sum += *item;
        }
    }
    sum
}

pub fn part_one_sum(input: &str) -> u32 {
    let input = str_to_vec(input);
    summate(&input, 1)
}

pub fn part_two_sum(input: &str) -> u32 {
    let input = str_to_vec(input);
    let halfway = input.len() / 2;
    summate(&input, halfway)
}

