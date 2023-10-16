use std::io;

fn main() {

    loop {

        let mut input = String::new();

        println!("Enter a positive integer (or q to quit): ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let n: usize = match input.trim() {
            "q" => break,
            num => num.parse().expect("Invalid number")
        };

        let divisors = find_divisors(n);

        println!("Divisors of {}: {:?}", n, divisors);

    }

}

fn find_divisors(n: usize) -> Vec<usize> {

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

    divisors.sort();
    divisors
}