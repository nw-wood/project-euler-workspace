#[cfg(test)]
mod project_euler_tests_1_100 {

    //========================================================================
    #[test]
    fn problem_5_smallest_multiple() {

        use std::error::Error;
        use std::fmt::Debug;
        use std::fmt;

        struct GcdError(String);
        impl Error for GcdError {}  //Specify GcdError shares behavior of the Error trait 
        impl Debug for GcdError {   //Since Error requires Debug, implement
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("GcdError").field(&self.0).finish()
            }
        }

        impl fmt::Display for GcdError {    //Since Error requires Display, implement
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "GCD Error: {}", self.0)
            }
        }

        fn gcd(input: Vec<u64>) -> Result<u64, GcdError> {
            if input.len() < 2 {
                return Err(GcdError("provide at least 2 inputs".to_string()));
            }
            Ok(1)
            //ex:   gcd(48, 18)
            //      gcd(18, 48 % 18)    = gcd(18, 12)
            //      gcd(12, 18 % 12)    = gcd(12, 6)
            //      gcd(6, 12 % 6)      = gcd(6, 0)
        }

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