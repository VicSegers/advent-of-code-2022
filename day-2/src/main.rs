use std::fs;

fn main() {
    let input = read_input();
    let games = parse_input(input);

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

fn part1(games: &Vec<(i32, i32)>) -> i32 {
    let mut total_score = 0;

    for game in games {
        let mut score = 0;

        score += game.1;

        if game.0 == game.1 {
            score += 3;
        } else if game.1 - 1 % 3 == game.0 % 3 {
            score += 6;
        }

        total_score += score;
    }

    return total_score;
}

fn part2(games: &Vec<(i32, i32)>) -> i32 {
    let mut total_score = 0;

    for game in games {
        if game.1 == 1 {
            let mut losing_move = game.0 - 1;

            if losing_move == 0 {
                losing_move = 3;
            }

            total_score += losing_move;
        } else if game.1 == 2 {
            total_score += game.0 + 3;
        } else {
            let mut winning_move = game.0 + 1;

            if winning_move == 4 {
                winning_move = 1;
            }

            total_score += winning_move + 6;
        }
    }

    return total_score;
}

fn parse_input(input: String) -> Vec<(i32, i32)> {
    let mut values: Vec<(i32, i32)> = vec![];

    for line in input.split("\n") {
        let mut words = line.split(" ");

        let first;

        match words.next() {
            Some("A") => first = 1,
            Some("B") => first = 2,
            Some("C") => first = 3,
            _ => continue
        }

        let second;

        match words.next() {
            Some("X") => second = 1,
            Some("Y") => second = 2,
            Some("Z") => second = 3,
            _ => continue
        }

        values.push((first, second));
    }

    return values;
}

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt").
        expect("Could not read file");

    return contents;
}
