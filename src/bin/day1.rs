fn main() {
    let input: Vec<_> = include_str!("day1.txt")
        .split('\n')
        .map(|str| str.parse::<i32>().unwrap())
        .collect();

    let number_of_increases = find_number_of_increases(&input);
    println!("The number of increases in the input file were: {}", number_of_increases);

    let number_of_window_increases = find_number_of_window_increases(&input, 3);
    println!("The number of increases for a 3 sized window in the input file were: {}", number_of_window_increases)
}

/// Find the number of times each measurement in the input file increases
fn find_number_of_increases(input: &Vec<i32>) -> i32 {
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

    return increases;
}

// Find the number of times a window of input value was larger than the previous window
fn find_number_of_window_increases(input: &Vec<i32>, window_size: usize) -> i32 {
    let mut increases = 0;
    let mut previous_sum = -1;
    let mut window_sum = 0;
    let mut i = 0;

    while i < input.len() {
        window_sum += input[i];

        if i >= window_size - 1 {
            if previous_sum >= 0 && window_sum > previous_sum {
                increases += 1;
            }

            previous_sum = window_sum;
            window_sum -= input[i - (window_size - 1)];
        }

        i += 1;
    }

    return increases;
}
