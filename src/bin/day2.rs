fn main() {
    let input = include_str!("day2.txt")
        .split("\n")
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            return Instruction {
                instruction: split[0],
                amount: split[1].parse::<i32>().unwrap()
            }
        })
        .collect::<Vec<_>>();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &Vec<Instruction>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    for instruction in input {
        match instruction.instruction {
            "forward" => horizontal += instruction.amount,
            "down" => vertical += instruction.amount,
            "up" => vertical -= instruction.amount,
            _ => println!("Failed to interpret direction: {}", instruction.instruction)
        }
    }

    return horizontal * vertical;
}

fn part2(input: &Vec<Instruction>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for instruction in input {
        match instruction.instruction {
            "forward" => {
                horizontal += instruction.amount;
                vertical += instruction.amount * aim
            },
            "down" => aim += instruction.amount,
            "up" => aim -= instruction.amount,
            _ => println!("Failed to interpret direction: {}", instruction.instruction)
        }
    }

    return horizontal * vertical;
}

struct Instruction<'a> {
    instruction: &'a str,
    amount: i32,
}