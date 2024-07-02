use std::io;


fn sum(numbers: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(numbers: &[i32]) -> f64 {
    let sum: f64 = sum(numbers) as f64;
    let length: f64 = numbers.len() as f64;
    sum / length
}


fn main() {
    let mut input: String = String::new();
    println!("\nEnter the length of the array:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let length: usize = input.trim().parse().expect("Invalid input");

    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..length {
        let mut input: String = String::new();
        println!("\nEnter number {} of {}:", i + 1, length);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let number: i32 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }

    let result: i32 = sum(&numbers);
    let average: f64 = average(&numbers);
    println!("\nThe sum is {}", result);
    println!("\nThe average is {}", average);
}
