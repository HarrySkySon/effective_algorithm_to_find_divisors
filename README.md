# effective_algorithm_to_find_divisors
There is a more effective algorithm to find divisors of a number than iterating from 1 to n.

This algorithm uses the fact that if d is a divisor of n, then so is n/d. Therefore, we only need to iterate from 1 to sqrt(n) to find all the divisors of n.
This program uses a for loop to iterate over all the integers from 1 to the square root of the input number n. For each integer, it checks if it is a divisor of n using the modulo operator %. If the remainder is zero, then the integer is a divisor, so it is added to a Vec of divisors. Additionally, if the integer is not equal to the square root of n, then n/i is also a divisor, so it is added to the Vec.
Finally, the program sorts the list of divisors and prints it to the console using Rust's println! macro.
