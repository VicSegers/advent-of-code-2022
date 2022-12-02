use std::fs;

fn main() {
    let input = read_input();
    let mut elves = parse_input(input);
    elves.sort();
    elves.reverse();

    println!("Part 1: {}", sum_highest(&elves, 1));
    println!("Part 2: {}", sum_highest(&elves, 3));
}

fn sum_highest(array: &Vec<i32>, amount: usize) -> i32 {
    let mut sum = 0;

    if array.len() < amount {
        println!("You got fucked");
        return 0;
    }

    for i in 0..amount {
        sum += array[i];
    }

    return sum;
}

fn parse_input(input: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![];
    let mut sum = 0;

    for line in input.split("\n") {
        if line == "" {
            numbers.push(sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<i32>().unwrap();
    }

    return numbers;
}

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt").
        expect("Could not read file");

    return contents;
}
