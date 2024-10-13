use std::error::Error;
use std::fmt::Debug;
use std::fmt;

//should leave my notes in here or seperate them out into a workspace thing or something

#[derive(Debug)]
struct _EulerError(String);
impl Error for _EulerError {}
impl fmt::Display for _EulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Euler Error: {}", self.0)
    }
}

#[cfg(test)]//STOP CLICKING THIS TEST
mod project_euler_tests_1_50 {
    use super::*;

    #[test]
    fn problem_9_special_pythagorean_triplet() {
        //brute force: .40s after breaks
        // for c in 1..499 as usize {
        //     for b in 1..499 as usize {
        //         if b >= c { break; }
        //         for a in 1..499 as usize {
        //             if a >=  b { break; }
        //             if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
        //                 println!("answer: {}", a * b * c);
        //             }
        //         }
        //     }
        // }

        //using euclid's formula, a = m^2-n^2, b = 2mn, c = m^2 + n^2
        //solves in 0.00s
        let mut d: Vec<usize> = vec![];
        for n in 2..=1000 / 2 {
            if 1000 % n == 0 {
                d.push(n);
            }
        }
        for m in &d{
            for n in &d {
                if m < n { break; }
                let (a, b, c) = (m.pow(2) - n.pow(2), 2 * m * n, m.pow(2) + n.pow(2));
                if a + b + c == 1000 {
                    println!("{}", a * b * c);
                }
            }
        }
    }
    //========================================================================
    #[test]
    fn problem_8_largest_product_in_a_series() {
        //there is a .window method for vectors that will make an efficient iterator out of all of this for a significant performance increase
        //also why bother multiplying just sum since the highest sum of all inputs is also the highest product
        //also skip 0's
        //lots of tricks left
        let full_series = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
        let mut series_list: Vec<&str> = vec![];
        let mut products: Vec<usize> = vec![];
        for index in 0..full_series.len()-1 {
            series_list.push(&full_series[index..index+13]);
            if index >= full_series.len() - 13 { break; }
        }
        let mut product = 1;
        for item in series_list {
            for i in 0..item.len() {
                if let Ok(n) = &item[i..i+1].parse::<usize>() { product *= n; }
            }
            products.push(product);
            product = 1;
        }
        let mut largest = 0;
        for item in products {
            if item > largest { largest = item }
        }
        println!("largest: {largest}");
    }
    //========================================================================
    #[test]
    fn problem_7_find_10001st_prime() {
        let mut n: usize = 3;
        let required: usize = 10001;
        let mut primes: Vec<usize> = vec![2];
        loop {
            let sqrt_of_n = (n as f64).sqrt().floor() as usize;
            for prime in &primes {
                if n % prime == 0 {
                    break;
                } else if *prime > sqrt_of_n  {
                    primes.push(n);
                    break;
                }
            }
            n += 2;
            if primes.len() == required { break; }
        }
        println!("answer: {}", primes.last().unwrap());
    }
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