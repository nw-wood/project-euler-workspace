use std::error::Error;
use std::fmt::Debug;
use std::fmt;

#[derive(Debug)]
struct _EulerError(String);
impl Error for _EulerError {}
impl fmt::Display for _EulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Euler Error: {}", self.0)
    }
}

#[cfg(test)]
mod project_euler_tests_1_50 {

    use super::*;
    //========================================================================
    #[test]
    fn problem_6_sum_square_difference() {
        let mut sum_of_squares = 0;
        let mut square_of_sums = 0;
        for i in 1..100 + 1 {
            sum_of_squares += i * i;
            square_of_sums += i;
        }
        square_of_sums *= square_of_sums;
        let result = square_of_sums - sum_of_squares;
        println!("result: {result}");
        assert_eq!(result, 25164150);
    }

    //========================================================================
    #[test]
    fn problem_5_smallest_multiple() {

        fn gcd(mut input: Vec<i64>) -> Result<i64, _EulerError> {

            if input.len() == 0 { return Err(_EulerError("provide at least 1 input".to_string())); }
            if input.len() == 1 { return Ok(input[0]); }

            input.sort();
            input.dedup();

            let mut a = input.pop().unwrap().abs();
            let mut b = input.pop().unwrap().abs();

            if !input.is_empty() { input.retain(|value| a % value != 0 || b % value != 0); }

            while a != 0 && b != 0 {
                if a > b {
                    a = a % b;
                } else {
                    b = b % a;
                }
            }

            if a != 0 { input.push(a); }
            else { input.push(b) }

            if input.len() > 1 { return gcd(input); }
            else {
                return Ok(input[0]);
            }
        }

        fn lcm(mut input: Vec<i64>) -> Result<i64, _EulerError> {
            //lcm(a, b) = |ab|/gcd(a,b)
            if input.len() < 2 {
                return Err(_EulerError("provide at least 2 numbers to find the least common multiple of".to_string()));
            }

            input.sort();
            input.dedup();
            input.retain(|item| *item != 0);

            let mut least = input.pop().unwrap().abs();

            while !input.is_empty() {

                let next = input.pop().unwrap().abs();

                match gcd(vec![least, next]) {

                    Ok(result) => {
                        least = least * next / result;

                    },
                    Err(e) => return Err(e),
                }
            }

            Ok(least)

        }

        assert_eq!(lcm(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]).unwrap(), 232792560);

    }
    
    //========================================================================
    #[test]
    fn problem_4_largest_palindrome_product() {
        let mut largest: usize = 0;
        for a in (100..1000).rev() {
            for b in (a..1000).rev() {
                let product = (a * b).to_string();
                let reverse: String = product.chars().rev().collect();
                if product == reverse && (a * b) > largest {
                    largest = a * b;
                }
            }
        }
        println!("largest: {largest}");
        assert_eq!(largest, 906609);
    }

    //========================================================================
    #[test]
    fn problem_3_largest_prime_factor() {
    
        let sqrt = (600_851_475_143.0 as f64).sqrt().round() as usize;
    
        let mut factors: Vec<usize> = vec![];
    
        for each in (1..sqrt).rev() {
            if 600_851_475_143 % each == 0 {
                println!("{each} x {} = {}", sqrt / each, sqrt);
                factors.push(each);
            }
        }
    
        println!("factors: {factors:?}");
    
        for factor in factors {
    
            println!("checking factor: {factor}");
    
            let mut factor_factors: Vec<usize> = vec![];
    
            for each in (1..factor / 2).rev() {
                if factor % each == 0 {
                    println!("...");
                    factor_factors.push(each);
                }
            }
    
            if factor_factors.len() < 2 {
                println!("{factor} was prime!");
                assert_eq!(factor, 6857);
                return;
            }
        }
    }

    //========================================================================
    #[test]
    fn problem_2_even_fibonacci_numbers() {

        let number = 4_000_000;

        let mut current: [usize; 2] = [1, 2];
    
        let mut even_sums: usize = 2;
    
        while current.iter().sum::<usize>() <= number {
    
            let sum: usize = current.iter().sum();
    
            if sum % 2 == 0 {
    
                even_sums += sum;
    
            }
    
            current[0] = current[1];
            current[1] = sum;
    
        }
    
        println!("sum of even fibs up to {number}: {even_sums}");
        assert_eq!({even_sums}, 4613732);
    
    }

    //========================================================================
    #[test]
    fn problem_1_multiples_of_3_or_5() {

        let mut sum: usize = 0; 
    
        for each in 1..1000 {
            if each % 3 == 0 || each % 5 == 0 { sum += each; }
        }
    
        println!("sum: {sum}");
        assert_eq!({sum}, 233168);
    }
}