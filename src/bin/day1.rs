fn main() {
    let input: Vec<_> = include_str!("day1.txt")
        .split('\n')
        .map(|str| str.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;

    let mut i = 1;
    let mut last_value = input[0];

    while i < input.len() {
        let current_value = input[i];
        if current_value > last_value {
            increases += 1;
        }

        last_value = current_value;
        i += 1;
    }

    println!("{}", increases)
}