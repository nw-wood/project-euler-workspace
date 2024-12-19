#[cfg(test)]
mod project_euler_tests_1_50 {
    #[test]
    fn problem_20_factorial_digit_sum() {
        
        use num_bigint::BigUint;

        let fact_n = (1..=100usize).fold(BigUint::from(1usize), |acc, x| acc * x);

        let digit_sum: u32 = fact_n.to_string()
            .chars()
            .filter_map(|char| char.to_digit(10))
            .sum();

        println!("{}", digit_sum);
    }


    #[test]
    fn problem_19_counting_sundays() {

        let mut day = 2; //0 sunday, 1 monday, 2 tuesday, jan 1 1901 is a tuesday
        let mut sundays = 0;

        for year in 1901..=2000 {
            for month in 1..=12 {
                if day == 0 {
                    sundays += 1;
                }
                day = (day + {
                    match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 => {
                            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                                29
                            } else {
                                28
                            }
                        },
                        _ => 0,
                    }
                }) % 7; //day = (day + days in that month) % 7, each time day is 0 increase sunday count since that indicates the starting day is sunday
            }
        }
        println!("sundays: {}", sundays);
    }

    #[test]
    fn problem_18_max_path_sum() {

        let mut triangle = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23]];

        //for each row starting from the 13th row (1 up from bottom, and moving up the triangle)
        for row in (0..triangle.len() - 1).rev() {
            //for each item in that row starting from left to right, initially 63 then 66, then 4
            for col in 0..triangle[row].len() {
                //add the value that is largest between the two items in the row after it
                triangle[row][col] += triangle[row + 1][col].max(triangle[row + 1][col + 1]);
            }
        }
        //all of the largest have been added up to the top of the triangle, print it
        println!("answer: {}", triangle[0][0]);
    }


    #[test]
    fn problem_17_number_letter_counts() {
        fn ones_to_word(input: i64) -> String {
            if input == 1 { "one".to_string() }
            else if input == 2 { "two".to_string() }
            else if input == 3 { "three".to_string() }
            else if input == 4 { "four".to_string() }
            else if input == 5 { "five".to_string() }
            else if input == 6 { "six".to_string() }
            else if input == 7 { "seven".to_string() }
            else if input == 8 { "eight".to_string() }
            else if input == 9 { "nine".to_string() }
            else { "".to_string() }
        }
        fn tens_to_word(input: i64) -> String {
            if input == 1 { "ten".to_string() }
            else if input == 2 { "twenty".to_string() }
            else if input == 3 { "thirty".to_string() }
            else if input == 4 { "forty".to_string() }
            else if input == 5 { "fifty".to_string() }
            else if input == 6 { "sixty".to_string() }
            else if input == 7 { "seventy".to_string() }
            else if input == 8 { "eighty".to_string() }
            else if input == 9 { "ninety".to_string() }
            else { "".to_string() }
        }
        fn hundreds_to_word(input: i64, and_a: i64, and_b: i64) -> String {
            if input != 0 {
                let one = ones_to_word(input);
                if and_a > 0 || and_b > 0 { one + "hundredand" }
                else { one + "hundred" }
            } else { "".to_string() }
        }
        fn thousands_to_word(input: i64) -> String {
            if input != 0 {
                let one = ones_to_word(input);
                one + "thousand"
            } else { "".to_string() }
        }
        let mut stack: String = String::new();
        for i in 1..=1000 {
            let ones = i % 10;
            let tens = (i /10) % 10;
            let hundreds = (i / 100) % 10;
            let thousands = (i / 1000) % 10;
            let mut teens = format!("{}{}", tens_to_word(tens), ones_to_word(ones));
            if teens == "tenone".to_string() { teens = "eleven".to_string() }
            else if teens == "tentwo".to_string() { teens = "twelve".to_string() }
            else if teens == "tenthree".to_string() { teens = "thirteen".to_string() }
            else if teens == "tenfour".to_string() { teens = "fourteen".to_string() }
            else if teens == "tenfive".to_string() { teens = "fifteen".to_string() }
            else if teens == "tensix".to_string() { teens = "sixteen".to_string() }
            else if teens == "tenseven".to_string() { teens = "seventeen".to_string() }
            else if teens == "teneight".to_string() { teens = "eighteen".to_string() }
            else if teens == "tennine".to_string() { teens = "nineteen".to_string() }
            stack = format!("{}{}{}{}", stack, thousands_to_word(thousands), hundreds_to_word(hundreds, tens, ones), teens);
        }
        println!("{}", stack.len());
    }



    #[test]
    fn problem_16_power_digit_sum() {
        use num_bigint::BigUint;
        let tpot = BigUint::new(vec![2]).pow(1000);
        let vpot = tpot.to_radix_be(10);
        let mut sum: u64 = 0;
        for n in vpot {
            sum += n as u64;
        }
        println!("answer: {sum}");
    }



    #[test]
    fn problem_15_lattice_paths() {
        use num_bigint::BigUint;
        fn factorial(input: usize) -> BigUint {
            let mut product: BigUint = BigUint::new(vec![1]);
            for n in 2..=input {
                product *= n;
            }
            product
        }
        let result = factorial(40) / (factorial(20) * factorial(20));
        println!("answer: {result}");
    }



    #[test]
    fn problem_14_longest_collatz_sequence() {
        let one_million: usize = 1_000_000;
        let mut longest_sequence: usize = 0;
        let mut longest_number: usize = 0;
        for i in (0..one_million).rev() {
            let mut n = i;
            let mut sequence_length: usize = 0;
            while n > 1 {
                if n % 2 == 0 {
                    n /= 2;
                } else {
                    n = 3 * n + 1;
                }
                sequence_length += 1;
            }
            if sequence_length > longest_sequence {
                longest_sequence = sequence_length;
                longest_number = i;
            }
        }
        println!("answer: {longest_number} with sequence length {longest_sequence}");
    }



    #[test]
    fn problem_13_large_sum() {
        use std::fs::File;
        use std::io::{self, BufRead};
        let file = File::open("./big_nums.txt").unwrap();
        let reader = io::BufReader::new(file);
        let mut str_list: Vec<String> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            str_list.push(line);
        }
        let mut carry: usize = 0;
        let mut big_sum: String = String::new();
        for digit in (0..50).rev() {
            let mut sum: usize = 0;
            for big_num in &str_list {
                let digit: usize = big_num[digit..digit+1].parse().unwrap();
                sum += digit;
            }
            sum += carry;
            let place = sum % 10;
            carry = sum / 10;
            big_sum = format!("{}{}", place, big_sum);
        }
        big_sum = format!("{}{}", carry, big_sum);
        println!("answer: {}", &big_sum[0..10]);
    }



    #[test]
    fn problem_12_triangle_number_with_500_divisors() {
        let mut n: usize = 1;
        let mut next: usize = 1;
        loop {
            next += 1;
            n += next;
            let mut divisors: Vec<usize> = vec![1];
            for i in 2..(n as f64).sqrt().floor() as usize {
                if n % i == 0 {
                    divisors.push(i);
                    if i != n / i { //remember to add division result to divisors list!
                        divisors.push(n / i);
                    }
                }
            }
            if divisors.len() > 500  {
                println!("answer: {n}");
                break;
            }
        }
    }



    #[test]
    fn problem_11_largest_product_in_grid() {
        let grid: Vec<usize> = vec![8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8, 49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0, 81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65, 52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91, 22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80, 24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50, 32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70, 67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21, 24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72, 21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95, 78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92, 16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57, 86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58, 19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40, 4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66, 88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69, 4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36, 20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16, 20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54, 1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48];
        let grid_rows: Vec<Vec<usize>> = grid.chunks(20).map(|chunk| chunk.to_vec()).collect();
        let mut largest: usize = 0;
        for (row_index, row) in grid_rows.iter().enumerate() {
            for (index, value) in row.iter().enumerate() {
                let (mut right, mut down, mut diag_right, mut diag_left) = (* value, * value, * value, * value);
                for i in 1..=3 {
                    if let Some(n) = row.get(index + i) { right *= n; }
                    if let Some(offset_row) = grid_rows.get(row_index + i) {
                        if let Some(n) = offset_row.get(index) { down *= n; }
                    }
                    if let Some(offset_row) = grid_rows.get(row_index + i) {
                        if let Some(n) = offset_row.get(index + i) { diag_right *= n; }
                    }
                    if let Some(offset_row) = grid_rows.get(row_index + i) {
                        if let Some(checked) = index.checked_sub(i) {
                            if let Some(n) = offset_row.get(checked) { diag_left *= n; }
                        }
                    }
                }
                if right > largest { largest = right; }
                if down > largest { largest = down; }
                if diag_right > largest { largest = diag_right; }
                if diag_left > largest { largest = diag_left; }
            }
        }
        println!("answer: {largest}");
    }



    #[test]
    fn problem_10_sum_first_2m_primes() {
        let mut n: usize = 3;
        let mut primes: Vec<usize> = vec![2];
        let mut sum: usize = 2;
        loop {
            let sqrt_of_n = (n as f64).sqrt().floor() as usize;
            for prime in &primes {
                if n % prime == 0 {
                    break;
                } else if *prime > sqrt_of_n  {
                    primes.push(n);
                    sum += n;
                    break;
                }
            }
            n += 2;
            if n > 2000000 { break; }
        }
        println!("answer: {sum}");
    }



    #[test]
    fn problem_9_special_pythagorean_triplet() {
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



    #[test]
    fn problem_8_largest_product_in_a_series() {
        //.window method, sum inputs instead of multiplying, skip 0's - think of more
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
 


    #[test]
    fn problem_5_smallest_multiple() {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct _EulerError(String);
        impl Error for _EulerError {}
        impl fmt::Display for _EulerError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Euler Error: {}", self.0)
            }
        }

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