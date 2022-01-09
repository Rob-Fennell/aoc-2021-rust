fn main() {
    let mut position = ShipPosition {
        horizontal: 0,
        vertical: 0,
    };

    include_str!("day2.txt")
        .split("\n")
        .map(|line| {
            return line.split(" ").collect::<Vec<&str>>();
        })
        .for_each(|element| {
            let direction = element[0];
            let value = element[1].parse::<i32>().unwrap();

            match direction {
                "forward" => position.horizontal += value,
                "down" => position.vertical += value,
                "up" => position.vertical -= value,
                _ => println!("Failed to interpret direction: {}", direction)
            }
        });

    println!("Horizontal: {}, Vertical: {}", position.horizontal, position.vertical);
    println!("Total: {}", position.horizontal * position.vertical);
}

struct ShipPosition {
    horizontal: i32,
    vertical: i32,
}
