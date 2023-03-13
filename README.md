# effective_algorithm_to_find_divisors
There is a more effective algorithm to find divisors of a number than iterating from 1 to n.

Here's a brief overview of what this code does:

- The program reads input from the user, which is assumed to be a positive integer.
- It finds the divisors of the input number using a loop that iterates from 1 up to the square root of the input number. 
- If a divisor is found, it is added to a vector of divisors. 
- The loop also checks if the divisor is not the same as the input number divided by the divisor, in which case that divisor is also added to the vector of divisors.
- Finally, the program sorts the vector of divisors in ascending order and prints the result.

This algorithm uses the fact that if d is a divisor of n, then so is n/d. Therefore, we only need to iterate from 1 to sqrt(n) to find all the divisors of n.
This program uses a for loop to iterate over all the integers from 1 to the square root of the input number n. For each integer, it checks if it is a divisor of n using the modulo operator %. If the remainder is zero, then the integer is a divisor, so it is added to a Vec of divisors. Additionally, if the integer is not equal to the square root of n, then n/i is also a divisor, so it is added to the Vec.
Finally, the program sorts the list of divisors and prints it to the console using Rust's println! macro.
