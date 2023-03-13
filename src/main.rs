use std::io;

fn main() {
    // Read input from user
    let mut input = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Find divisors
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }

    // Print results
    divisors.sort();
    println!("Divisors of {}: {:?}", n, divisors);
}
